import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [1, 2];
	$$renderer.push(`<button>add</button> <!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		$$renderer.push(`<span>${$.escape(i)}:${$.escape(item)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
}
