import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	$$renderer.push(`<template shadowrootmode="open"><p>${$.escape(x)}</p></template>`);
}
