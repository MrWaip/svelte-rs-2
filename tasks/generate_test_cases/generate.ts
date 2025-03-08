import { compile } from "npm:svelte@5.22.6/compiler";

const text = await Deno.readTextFile(Deno.args[0]);

const result = compile(text, {
  discloseVersion: false,
  dev: false,
  name: "App",
  modernAst: true,
  runes: true,
});

console.log(result.js.code);
