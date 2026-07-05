import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [1];
	$$renderer.push(`<div class="a svelte-10ib7zp"><!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let x = each_array[$$index];
		$$renderer.push(`<div class="b svelte-10ib7zp">${$.escape(x)}</div>`);
	}
	$$renderer.push(`<!--]--></div>`);
}
