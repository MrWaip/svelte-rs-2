import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [[
		1,
		2,
		3
	]];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { length, [length - 1]: last, [Math.floor(length / 2)]: mid } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(last)}${$.escape(mid)}${$.escape(length)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
