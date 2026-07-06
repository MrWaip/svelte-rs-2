import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{
		a: 1,
		b: 2
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { a, b } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
