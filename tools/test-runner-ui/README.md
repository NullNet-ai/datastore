# Test Runner UI

Web UI for running and inspecting Rust store tests.

## Run

From the tools directory:

```bash
cd tools/test-runner-ui
npm install
node server.js
```

Then open http://localhost:3847

**Development (reload on changes):**
```bash
npm run dev
```

## Features

- **Test list**: All tests from the store package
- **Filter**: Type to filter tests by name
- **Run all**: Execute all tests
- **Run selected**: Run only checked tests
- **Run one**: Double-click a test to run it alone
- **Results**: Pass/fail count, criteria, output, assertion failures
- **Detail view**: Click a result row to see full criteria, stdout, and failure message

## Requirements

- Node.js 18+ (for native fetch and ESM)
- Rust/Cargo with the store package buildable
