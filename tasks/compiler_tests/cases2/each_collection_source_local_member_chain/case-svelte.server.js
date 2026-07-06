import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const obj = { list: [
		1,
		2,
		3
	] };
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(obj.list);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<span>${$.escape(item)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
}
