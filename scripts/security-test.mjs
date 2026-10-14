import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

// Exercise the v1 upstream fix's non-expanding-group contract through the
// installed package and its minimatch consumer, without a long-running hang.
// https://github.com/juliangruber/brace-expansion/commit/d74e63030c012e3b7ae81657b8d665619cd51b95
const source = String.raw`
  const assert = require('node:assert/strict');
  const expand = require('brace-expansion');
  const minimatch = require('minimatch');
  const pattern = Array(5000).fill('{}').join(',');
  assert.deepEqual(expand(pattern), [pattern]);
  assert.deepEqual(minimatch.braceExpand(pattern), [pattern]);
  assert.deepEqual(expand('file{01..03}.wasm'), ['file01.wasm', 'file02.wasm', 'file03.wasm']);
  assert.deepEqual(expand('x{{a,b}}y'), ['x{a}y', 'x{b}y']);
  assert.deepEqual(expand('a{},b}c'), ['a}c', 'abc']);
  assert.deepEqual(expand('$' + '{a,b}'), ['$' + '{a,b}']);
`;

const result = spawnSync(process.execPath, [
  "--max-old-space-size=64",
  "--stack-size=256",
  "-e", source
], {
  cwd: fileURLToPath(new URL("../", import.meta.url)),
  timeout: 10000,
  killSignal: "SIGKILL",
  maxBuffer: 16 * 1024,
  encoding: "utf8"
});

assert.ifError(result.error);
assert.equal(result.status, 0, result.stderr || `Child terminated with ${result.signal}`);
console.log("Security regression passed (5,000 groups; direct and minimatch expansion)");
