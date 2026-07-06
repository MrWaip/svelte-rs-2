import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
		let item = each_array[$$index_1];
		$$renderer.push(`<!--[-->`);
		const each_array_1 = $.ensure_array_like(item.kids);
		for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
			let kid = each_array_1[$$index];
			$$renderer.push(`<p>${$.escape(kid)}</p>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items });
}
