import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const fn = (node, options) => ({});
	let obj = $.derived(() => ({ inner: fn }));
	$$renderer.push(`<div></div>`);
}
