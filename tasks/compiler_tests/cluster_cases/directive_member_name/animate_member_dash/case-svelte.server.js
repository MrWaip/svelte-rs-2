import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.derived(() => a);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like([]);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let i = each_array[$$index];
		$$renderer.push(`<div></div>`);
	}
	$$renderer.push(`<!--]-->`);
}
