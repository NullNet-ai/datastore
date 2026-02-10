import { createServer } from 'http';
import { spawn } from 'child_process';
import { readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const WORKSPACE_ROOT = join(__dirname, '..', '..');
const STORE_PACKAGE = 'store';
const TEST_LIST_CACHE_TTL_MS = 5 * 60 * 1000; // 5 minutes
let testListCache = { tests: null, at: 0 };

const MIME = {
  '.html': 'text/html',
  '.css': 'text/css',
  '.js': 'application/javascript',
  '.ico': 'image/x-icon',
};

function serveStatic(req, res, pathname) {
  const requested = pathname === '/' ? '/index.html' : pathname;
  const file = join(__dirname, 'public', requested);
  if (!existsSync(file) || !file.startsWith(join(__dirname, 'public'))) {
    return false;
  }
  const ext = requested.split('.').pop();
  res.writeHead(200, { 'Content-Type': MIME[`.${ext}`] || 'text/plain' });
  res.end(readFileSync(file));
  return true;
}

function parseCargoTestOutput(stdout, stderr) {
  const combined = stdout + '\n' + stderr;
  const results = [];

  // Pattern 1: "test NAME ..." followed by output lines and "ok"/"FAILED" on next line(s)
  const blockRegex = /test\s+([\w:]+)\s+\.\.\.\s*\n([\s\S]*?)(?=\ntest\s+[\w:]|\ntest result:|$)/g;
  let m;
  while ((m = blockRegex.exec(combined)) !== null) {
    const name = m[1].trim();
    const block = m[2] || '';
    const statusMatch = block.match(/(?:^|\n)(ok|FAILED|failed|ignored)\s*$/m);
    const status = statusMatch
      ? (statusMatch[1].toLowerCase() === 'failed' || statusMatch[1] === 'FAILED' ? 'FAILED' : statusMatch[1])
      : 'ok';
    const outputLines = block.replace(/(?:^|\n)(ok|FAILED|failed|ignored)\s*$/m, '').trim();
    const panicMatch = block.match(/thread\s+'[^']+'\s+panicked at (?:'([^']*(?:\\'[^']*)*)'|"([^"]*)")/);
    const failure = panicMatch ? (panicMatch[1] || panicMatch[2] || '').replace(/\\'/g, "'") : '';
    results.push({ name, status, output: outputLines, failure });
  }

  // Pattern 2 fallback: "test NAME ... ok" on single line (no captured output)
  if (results.length === 0) {
    const lineRegex = /test\s+([\w:]+)\s+\.\.\.\s+(ok|FAILED|failed|ignored)(?:\s|$)/g;
    while ((m = lineRegex.exec(combined)) !== null) {
      const status = m[2].toLowerCase() === 'failed' ? 'FAILED' : m[2];
      results.push({ name: m[1].trim(), status, output: '', failure: '' });
    }
  }


  // For failures (when block regex didn't capture), extract from ---- blocks
  const failureBlocks = combined.split(/\n----\s+/);
  for (let i = 1; i < failureBlocks.length; i++) {
    const block = failureBlocks[i];
    const headerMatch = block.match(/^([^\s]+)\s+stdout\s+----\s*\n([\s\S]*?)(?=\nthread\s+'|\n----\s+|$)/);
    const panicMatch = block.match(/thread\s+'([^']+)'\s+panicked at (?:'([^']*(?:\\'[^']*)*)'|"([^"]*)")/);
    const testName = panicMatch ? panicMatch[1] : (headerMatch ? headerMatch[1] : '');
    const output = headerMatch ? headerMatch[2].trim() : '';
    const failure = panicMatch ? (panicMatch[2] || panicMatch[3] || '').replace(/\\'/g, "'") : '';
    const r = results.find((t) => t.name === testName);
    if (r) {
      if (!r.output) r.output = output;
      if (!r.failure) r.failure = failure;
    }
  }

  const summaryMatches = [...combined.matchAll(/test result:\s*(?:ok|FAILED)\.\s*(\d+)\s+passed;\s*(\d+)\s+failed/g)];
  const summaryMatch = summaryMatches.length > 0 ? summaryMatches[summaryMatches.length - 1] : null;
  const summary = summaryMatch
    ? { passed: parseInt(summaryMatch[1], 10), failed: parseInt(summaryMatch[2], 10) }
    : { passed: results.filter((r) => r.status === 'ok').length, failed: results.filter((r) => r.status === 'FAILED').length };

  return { results, summary, raw: combined };
}

async function listTests() {
  return new Promise((resolve, reject) => {
    const proc = spawn('cargo', ['test', '-p', STORE_PACKAGE, '--', '--list'], {
      cwd: WORKSPACE_ROOT,
      stdio: ['ignore', 'pipe', 'pipe'],
      shell: process.platform === 'win32',
    });
    let stdout = '';
    let stderr = '';
    proc.stdout?.on('data', (d) => (stdout += d.toString()));
    proc.stderr?.on('data', (d) => (stderr += d.toString()));
    proc.on('close', (code) => {
      if (code !== 0) {
        reject(new Error(stderr || stdout || `cargo test --list exited with ${code}`));
        return;
      }
      const skip = /^(Running|Compiling|Finished|Downloading|Documenting|Checking)/;
      const tests = (stdout + stderr)
        .split('\n')
        .map((l) => l.trim())
        .filter((l) => l && !skip.test(l))
        .map((l) => (l.endsWith(': test') ? l.slice(0, -6) : l));
      resolve(tests);
    });
    proc.on('error', reject);
  });
}

function runTestsProcess(testFilter, useNocapture) {
  return new Promise((resolve, reject) => {
    const args = ['test', '-p', STORE_PACKAGE, '--'];
    if (useNocapture) args.push('--nocapture');
    if (testFilter) args.push(testFilter);
    const proc = spawn('cargo', args, {
      cwd: WORKSPACE_ROOT,
      stdio: ['ignore', 'pipe', 'pipe'],
      shell: process.platform === 'win32',
    });
    let stdout = '';
    let stderr = '';
    proc.stdout?.on('data', (d) => (stdout += d.toString()));
    proc.stderr?.on('data', (d) => (stderr += d.toString()));
    proc.on('close', (code) => {
      const parsed = parseCargoTestOutput(stdout, stderr);
      parsed.exitCode = code;
      resolve({ ...parsed, stderr });
    });
    proc.on('error', reject);
  });
}

async function runTests(testFilter = null) {
  const result = await runTestsProcess(testFilter, true);
  if (result.exitCode !== 0 && result.stderr.includes('unexpected argument') && (result.stderr.includes('nocapture') || result.stderr.includes('no-capture'))) {
    return runTestsProcess(testFilter, false);
  }
  return result;
}

const server = createServer(async (req, res) => {
  const url = new URL(req.url, `http://${req.headers.host}`);
  const pathname = url.pathname;

  if (pathname.startsWith('/api/')) {
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Access-Control-Allow-Origin', '*');
    try {
      if (pathname === '/api/tests' && req.method === 'GET') {
        const forceRefresh = url.searchParams.get('refresh') === '1';
        const now = Date.now();
        if (!forceRefresh && testListCache.tests && now - testListCache.at < TEST_LIST_CACHE_TTL_MS) {
          res.end(JSON.stringify({ tests: testListCache.tests }));
          return;
        }
        const tests = await listTests();
        testListCache = { tests, at: Date.now() };
        res.end(JSON.stringify({ tests }));
        return;
      }
      if (pathname === '/api/run' && req.method === 'POST') {
        let body = '';
        for await (const chunk of req) body += chunk;
        const { test: testFilter, tests: testNames } = JSON.parse(body || '{}');
        if (Array.isArray(testNames) && testNames.length > 0) {
          const runs = await Promise.all(testNames.map((name) => runTests(name)));
          const allResults = [];
          let totalPassed = 0;
          let totalFailed = 0;
          let raw = '';
          for (const r of runs) {
            allResults.push(...(r.results || []));
            totalPassed += (r.summary?.passed || 0);
            totalFailed += (r.summary?.failed || 0);
            raw += (r.raw || '') + '\n\n';
          }
          const merged = {
            results: allResults,
            summary: { passed: totalPassed, failed: totalFailed },
            raw: raw.trim(),
            exitCode: allResults.some((t) => t.status === 'FAILED') ? 1 : 0,
          };
          res.end(JSON.stringify(merged));
        } else {
          const result = await runTests(testFilter || null);
          res.end(JSON.stringify(result));
        }
        return;
      }
      res.writeHead(404);
      res.end(JSON.stringify({ error: 'Not found' }));
    } catch (err) {
      res.writeHead(500);
      res.end(JSON.stringify({ error: String(err.message) }));
    }
    return;
  }

  if (serveStatic(req, res, pathname)) return;
  res.writeHead(404);
  res.end('Not found');
});

const PORT = 3847;
server.listen(PORT, () => {
  console.log(`Test Runner UI: http://localhost:${PORT}`);
  listTests()
    .then((t) => {
      testListCache = { tests: t, at: Date.now() };
      console.log(`Pre-warmed test list (${t.length} tests)`);
    })
    .catch(() => {});
});
