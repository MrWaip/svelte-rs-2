import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { width = 10, area = Math.max(width, 0) } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(width)}${$.escape(area)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
