import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		const { a: [b, c] } = item;
		$$renderer.push(`<p>${$.escape(b)} ${$.escape(c)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
