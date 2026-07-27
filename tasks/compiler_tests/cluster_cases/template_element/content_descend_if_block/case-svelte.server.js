import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cond } = $$props;
	$$renderer.push(`<template>`);
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`a`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></template>`);
}
