import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<template id="t1"><div>foo</div></template>`);
}
