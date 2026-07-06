import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.derived(() => a);
	$$renderer.push(`<div></div>`);
}
