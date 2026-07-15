import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { loading, items } = $$props;
	if (loading) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<span>${$.escape(item)}</span>`);
		}
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--[-->`);
		const each_array_1 = $.ensure_array_like(items);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let item = each_array_1[$$index_1];
			$$renderer.push(`<div>${$.escape(item)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
}
