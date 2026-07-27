import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<template><p><!--[-->`);
	$.slot($$renderer, $$props, "default", {}, null);
	$$renderer.push(`<!--]--></p></template>`);
}
