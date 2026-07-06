import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const k = "z";
	let items = [{ z: 1 }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { [k]: v } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(v)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
