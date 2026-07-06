import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { items = [] } = $$props;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let item = each_array[i];
		$$renderer.push(`<div>${$.escape(i)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
