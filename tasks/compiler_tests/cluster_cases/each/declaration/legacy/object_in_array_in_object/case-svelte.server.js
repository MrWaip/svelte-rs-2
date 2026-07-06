import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ outer: [{ inner: 1 }] }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { outer: [{ inner }] } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(inner)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
