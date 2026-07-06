import * as $ from "svelte/internal/server";
export default function Modal_1($$renderer) {
	let items = [];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let Modal = each_array[$$index];
		$$renderer.push(`<p>${$.escape(Modal)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
