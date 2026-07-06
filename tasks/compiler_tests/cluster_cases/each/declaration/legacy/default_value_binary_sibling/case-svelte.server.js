import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { w = 1, x = w * 2 } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(w)}${$.escape(x)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
