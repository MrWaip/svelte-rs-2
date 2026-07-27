import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	$$renderer.push(`<template><p>${$.escape(x)}</p></template>`);
}
