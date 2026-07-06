import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let arr = [
		1,
		2,
		3
	];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(arr);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let o = each_array[$$index];
		$$renderer.push(`<button>${$.escape(o)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
