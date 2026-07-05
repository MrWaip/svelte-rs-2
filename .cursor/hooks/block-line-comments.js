#!/usr/bin/env node
let buf = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (c) => { buf += c; });
process.stdin.on('end', () => {
  let data;
  try { data = JSON.parse(buf); } catch { process.exit(0); }
  const inp = data.tool_input || data.toolInput || data.arguments || {};
  const parts = [
    inp.new_string,
    inp.contents,
    inp.content,
    ...((inp.edits || []).map((e) => e && e.new_string)),
  ].filter((s) => typeof s === 'string');
  const re = /^\s*\/\/.*$/m;
  for (const p of parts) {
    if (re.test(p)) {
      process.stderr.write('Blocked: // line comments forbidden (see CLAUDE.md). Strip them.\n');
      process.exit(2);
    }
  }
  process.exit(0);
});
