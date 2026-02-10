const API = '/api';
const $ = (id) => document.getElementById(id);

let tests = [];
let lastResult = null;
let selectedTests = new Set();
let testResults = [];
let currentResultIndex = -1;
let resultFilter = 'all'; // 'all' | 'passed' | 'failed'

async function fetchTests(forceRefresh = false) {
  const url = forceRefresh ? `${API}/tests?refresh=1` : `${API}/tests`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(await res.text());
  const data = await res.json();
  tests = data.tests || [];
  return tests;
}

async function runTests(filter = null) {
  const res = await fetch(`${API}/run`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ test: filter }),
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function runTestsBulk(testNames) {
  const res = await fetch(`${API}/run`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ tests: testNames }),
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

function shortName(full) {
  const parts = full.split('::');
  return parts.length > 2 ? parts.slice(-2).join('::') : full;
}

function escapeHtml(s) {
  const div = document.createElement('div');
  div.textContent = s;
  return div.innerHTML;
}

function normalizeStatus(s) {
  return s === 'ok' ? 'passed' : 'failed';
}

function extractTestNamesFromRaw(raw) {
  const names = [];
  const re = /test\s+([\w:]+)\s+\.\.\./g;
  let m;
  while ((m = re.exec(raw)) !== null) names.push(m[1].trim());
  return names;
}

function mapApiResultToUi(r, raw, duration) {
  return {
    name: r.name,
    status: normalizeStatus(r.status),
    criteria: r.status === 'ok' ? 'All assertions passed.' : (r.failure || 'Test panicked.'),
    input: `Test: ${r.name}\n\nInput (payload, params) is in the test source. For constructors: src/providers/queries/find/constructors/constructors_test.rs`,
    output: r.output || '(no stdout captured)',
    failure: r.failure || '',
    rawOutput: (raw || '').slice(0, 15000),
    duration: duration ? `${duration}ms` : '—',
  };
}

function renderTestList() {
  const list = $('test-list');
  const filterVal = ($('filter')?.value || '').toLowerCase();
  const filtered = tests.filter((t) => t.toLowerCase().includes(filterVal));
  $('test-count').textContent = `${filtered.length} test${filtered.length !== 1 ? 's' : ''}`;

  list.innerHTML = filtered
    .map((name) => {
      const status = lastResult?.results?.find((r) => r.name === name)?.status;
      const uiStatus = status ? normalizeStatus(status) : '';
      return `
        <li class="${uiStatus}" data-name="${escapeHtml(name)}">
          <input type="checkbox" class="test-checkbox" ${selectedTests.has(name) ? 'checked' : ''} data-name="${escapeHtml(name)}" />
          <span class="test-name" title="${escapeHtml(name)}">${shortName(name)}</span>
          ${uiStatus ? `<span class="test-status-icon ${uiStatus}"></span>` : ''}
        </li>`;
    })
    .join('');

  list.querySelectorAll('li').forEach((el) => {
    const name = el.dataset.name;
    el.addEventListener('click', (e) => {
      if (e.target.classList.contains('test-checkbox')) return;
      el.classList.toggle('selected');
      selectedTests.has(name) ? selectedTests.delete(name) : selectedTests.add(name);
      const cb = el.querySelector('.test-checkbox');
      if (cb) cb.checked = selectedTests.has(name);
    });
    el.addEventListener('dblclick', (e) => {
      if (e.target.classList.contains('test-checkbox')) return;
      e.preventDefault();
      doRunSelected([name]);
    });
  });

  list.querySelectorAll('.test-checkbox').forEach((cb) => {
    cb.addEventListener('change', (e) => {
      e.stopPropagation();
      const name = cb.dataset.name;
      cb.checked ? selectedTests.add(name) : selectedTests.delete(name);
      cb.closest('li')?.classList.toggle('selected', cb.checked);
    });
  });
}

function updateSummary(result) {
  lastResult = result;
  $('summary-placeholder').classList.add('hidden');
  $('summary-content').classList.remove('hidden');
  const { summary = {}, results = [] } = result;
  $('stat-passed').textContent = `${summary.passed ?? 0} passed`;
  $('stat-failed').textContent = `${summary.failed ?? 0} failed`;
  $('stat-duration').textContent = result.duration ? `${result.duration}ms` : '—';
  const criteria = [];
  if (summary.passed > 0) criteria.push(`${summary.passed} tests passed`);
  if (summary.failed > 0) criteria.push(`${summary.failed} tests failed`);
  $('summary-criteria').textContent = criteria.join('. ') || 'No results yet.';
}

function renderResultsList() {
  const filtered = resultFilter === 'all'
    ? testResults
    : testResults.filter((r) => r.status === resultFilter);

  $('filter-passed')?.classList.toggle('active', resultFilter === 'passed');
  $('filter-failed')?.classList.toggle('active', resultFilter === 'failed');
  $('filter-all')?.classList.toggle('active', resultFilter === 'all');

  const list = $('results-list');
  list.innerHTML = filtered
    .map((r) => {
      const i = testResults.indexOf(r);
      return `
      <div class="result-item" data-index="${i}">
        <div class="result-item-status ${r.status}">${r.status === 'passed' ? '✓' : '✗'}</div>
        <div class="result-item-info">
          <div class="result-item-name">${shortName(r.name)}</div>
          <div class="result-item-criteria">${escapeHtml((r.criteria || '').slice(0, 80))}</div>
        </div>
        <div class="result-item-arrow">→</div>
      </div>`;
    })
    .join('');

  list.querySelectorAll('.result-item').forEach((el) => {
    el.addEventListener('click', () => showResultDetail(parseInt(el.dataset.index, 10)));
  });
}

function showResultsOverview(results, duration) {
  testResults = results;
  resultFilter = 'all';
  $('empty-state').classList.add('hidden');
  $('results-nav').classList.add('hidden');
  $('result-detail').classList.add('hidden');
  $('results-overview').classList.remove('hidden');

  const passed = results.filter((r) => r.status === 'passed').length;
  const failed = results.filter((r) => r.status === 'failed').length;

  $('overview-passed').textContent = passed;
  $('overview-failed').textContent = failed;
  $('overview-total').textContent = results.length;

  $('overview-header').querySelector('h2').textContent = 'Test Results';
  $('nav-title').textContent = 'Test Result';

  renderResultsList();
}

function setResultFilter(filter) {
  resultFilter = filter;
  if (testResults.length > 0) renderResultsList();
}

function showResultDetail(index) {
  currentResultIndex = index;
  const r = testResults[index];
  if (!r) return;

  $('results-overview').classList.add('hidden');
  $('results-nav').classList.remove('hidden');
  $('result-detail').classList.remove('hidden');

  $('nav-title').textContent = r.name;
  $('nav-position').textContent = `${index + 1} / ${testResults.length}`;
  $('nav-prev').disabled = index === 0;
  $('nav-next').disabled = index === testResults.length - 1;

  $('result-title').textContent = r.name;
  $('result-badge').textContent = r.status === 'passed' ? 'Passed' : 'Failed';
  $('result-badge').className = `badge ${r.status}`;
  $('result-meta').textContent = `Duration: ${r.duration || '—'}`;

  $('result-input').textContent = r.input || '';
  $('result-output').textContent = r.output || '';
  $('result-criteria').textContent = r.criteria || '';
  $('result-failure').textContent = r.failure || '';
  $('result-raw').textContent = r.rawOutput || '';

  $('section-failure').classList.toggle('hidden', !r.failure);
}

$('nav-back')?.addEventListener('click', () => {
  if (testResults.length > 0) {
    showResultsOverview(testResults);
    $('nav-title').textContent = 'Test Result';
  }
});
$('nav-prev')?.addEventListener('click', () => {
  if (currentResultIndex > 0) showResultDetail(currentResultIndex - 1);
});
$('nav-next')?.addEventListener('click', () => {
  if (currentResultIndex < testResults.length - 1) showResultDetail(currentResultIndex + 1);
});

function setRunning(running) {
  $('run-all').disabled = running;
  $('run-selected').disabled = running;
  const btn = $('run-all');
  if (running) {
    btn.dataset.original = btn.textContent;
    btn.innerHTML = '<span class="spinner" style="display:inline-block;width:14px;height:14px;border:2px solid #e2e8f0;border-top-color:#3b82f6;border-radius:50%;animation:spin 1s linear infinite;vertical-align:middle;margin-right:6px;"></span> Running...';
  } else {
    btn.textContent = btn.dataset.original || 'Run All Tests';
  }
}

async function doRun(filter = null) {
  setRunning(true);
  const start = Date.now();
  try {
    const result = await runTests(filter);
    result.duration = Date.now() - start;
    updateSummary(result);

    let uiResults = (result.results || []).map((r) =>
      mapApiResultToUi(r, result.raw, result.duration)
    );

    // Fallback: summary says tests ran but parser returned no results
    const { summary = {} } = result;
    const totalRan = (summary.passed || 0) + (summary.failed || 0);
    if (uiResults.length === 0 && totalRan > 0 && result.raw) {
      const names = extractTestNamesFromRaw(result.raw);
      const status = (summary.failed || 0) > 0 ? 'failed' : 'passed';
      uiResults = (names.length > 0 ? names : ['Test run (parsing failed)']).map((name) => ({
        name,
        status,
        criteria: `${summary.passed || 0} passed, ${summary.failed || 0} failed`,
        input: `Test: ${name}\n\nInput (payload, params) is in the test source.`,
        output: '',
        failure: '',
        rawOutput: result.raw,
        duration: result.duration ? `${result.duration}ms` : '—',
      }));
    } else if (uiResults.length > 0 && uiResults.length < totalRan && result.raw) {
      const namesFromRaw = extractTestNamesFromRaw(result.raw);
      const haveNames = new Set(uiResults.map((r) => r.name));
      const missingNames = namesFromRaw.filter((n) => !haveNames.has(n));
      const havePassed = uiResults.filter((r) => r.status === 'passed').length;
      const needPassed = Math.max(0, (summary.passed || 0) - havePassed);
      missingNames.forEach((name, i) => {
        const status = i < needPassed ? 'passed' : 'failed';
        uiResults.push({
          name,
          status,
          criteria: status === 'passed' ? 'All assertions passed.' : 'Test panicked or assertion failed.',
          input: `Test: ${name}\n\nInput (payload, params) is in the test source.`,
          output: '',
          failure: '',
          rawOutput: result.raw,
          duration: result.duration ? `${result.duration}ms` : '—',
        });
      });
    }

    if (uiResults.length > 0) {
      showResultsOverview(uiResults, result.duration);
    } else {
      $('empty-state').classList.remove('hidden');
      $('results-overview').classList.add('hidden');
      $('result-detail').classList.add('hidden');
      $('results-nav').classList.add('hidden');
      const hint = $('empty-state').querySelector('.hint');
      if (hint && result.raw) {
        hint.innerHTML = `No parsed results. <a href="#" id="show-raw">Show raw cargo output</a>`;
        $('show-raw')?.addEventListener('click', (e) => {
          e.preventDefault();
          alert(result.raw.slice(0, 3000));
        });
      }
    }
    renderTestList();
  } catch (e) {
    alert('Error: ' + e.message);
  } finally {
    setRunning(false);
  }
}

async function doRunSelected(testNames) {
  setRunning(true);
  const start = Date.now();
  try {
    const result = await runTestsBulk(testNames);
    result.duration = Date.now() - start;
    updateSummary(result);

    let uiResults = (result.results || []).map((r) =>
      mapApiResultToUi(r, result.raw, result.duration)
    );
    const { summary = {} } = result;
    const totalRan = (summary.passed || 0) + (summary.failed || 0);
    if (uiResults.length === 0 && totalRan > 0 && result.raw) {
      const names = extractTestNamesFromRaw(result.raw);
      const status = (summary.failed || 0) > 0 ? 'failed' : 'passed';
      uiResults = (names.length > 0 ? names : ['Test run (parsing failed)']).map((name) => ({
        name,
        status,
        criteria: `${summary.passed || 0} passed, ${summary.failed || 0} failed`,
        input: `Test: ${name}\n\nInput (payload, params) is in the test source.`,
        output: '',
        failure: '',
        rawOutput: result.raw,
        duration: result.duration ? `${result.duration}ms` : '—',
      }));
    } else if (uiResults.length > 0 && uiResults.length < totalRan && result.raw) {
      const namesFromRaw = extractTestNamesFromRaw(result.raw);
      const haveNames = new Set(uiResults.map((r) => r.name));
      const missingNames = namesFromRaw.filter((n) => !haveNames.has(n));
      const havePassed = uiResults.filter((r) => r.status === 'passed').length;
      const haveFailed = uiResults.filter((r) => r.status === 'failed').length;
      const needPassed = Math.max(0, (summary.passed || 0) - havePassed);
      const needFailed = Math.max(0, (summary.failed || 0) - haveFailed);
      missingNames.forEach((name, i) => {
        const status = i < needPassed ? 'passed' : 'failed';
        uiResults.push({
          name,
          status,
          criteria: status === 'passed' ? 'All assertions passed.' : 'Test panicked or assertion failed.',
          input: `Test: ${name}\n\nInput (payload, params) is in the test source.`,
          output: '',
          failure: '',
          rawOutput: result.raw,
          duration: result.duration ? `${result.duration}ms` : '—',
        });
      });
    }
    if (uiResults.length > 0) showResultsOverview(uiResults, result.duration);
    else {
      $('empty-state').classList.remove('hidden');
      $('results-overview').classList.add('hidden');
      $('result-detail').classList.add('hidden');
    }
    renderTestList();
  } catch (e) {
    alert('Error: ' + e.message);
  } finally {
    setRunning(false);
  }
}

$('run-all').addEventListener('click', () => doRun(null));
$('run-selected').addEventListener('click', () => {
  if (selectedTests.size === 0) {
    alert('Select at least one test.');
    return;
  }
  doRunSelected([...selectedTests]);
});

$('refresh-list').addEventListener('click', async () => {
  try {
    await fetchTests(true);
    renderTestList();
  } catch (e) {
    alert('Error loading tests: ' + e.message);
  }
});

$('filter')?.addEventListener('input', () => renderTestList());

$('filter-passed')?.addEventListener('click', () => setResultFilter('passed'));
$('filter-failed')?.addEventListener('click', () => setResultFilter('failed'));
$('filter-all')?.addEventListener('click', () => setResultFilter('all'));

$('select-all').addEventListener('change', (e) => {
  const filterVal = ($('filter')?.value || '').toLowerCase();
  const filtered = tests.filter((t) => t.toLowerCase().includes(filterVal));
  if (e.target.checked) filtered.forEach((t) => selectedTests.add(t));
  else filtered.forEach((t) => selectedTests.delete(t));
  renderTestList();
});

async function init() {
  try {
    await fetchTests();
    renderTestList();
  } catch (e) {
    $('test-list').innerHTML = `<li style="color:var(--color-danger);">Failed to load tests: ${escapeHtml(e.message)}</li>`;
  }
}

init();
