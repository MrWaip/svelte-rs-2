import { compile } from "svelte/compiler";
import { readFileSync } from "node:fs";

const files = JSON.parse(readFileSync("/dev/stdin", "utf8"));
const results = {};
for (const file of files) {
  const text = readFileSync(file, "utf8");
  const result = compile(text, {
    discloseVersion: false,
    dev: false,
    name: "App",
    modernAst: true,
    runes: true,
  });
  results[file] = result.js.code;
}
console.log(JSON.stringify(results));
