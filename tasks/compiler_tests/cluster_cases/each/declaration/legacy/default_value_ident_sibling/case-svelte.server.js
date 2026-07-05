import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let array = [{
		a: 1,
		c: 2
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(array);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { a, b = c, c } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
