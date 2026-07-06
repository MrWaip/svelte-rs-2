import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{
		p: [1, 2],
		q: [3, 4]
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { p: [a, b], q: [c, d] } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
