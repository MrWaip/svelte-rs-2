import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [[[1, 2], [3, 4]]];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let [[a, b], [c, d]] = each_array[$$index];
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
