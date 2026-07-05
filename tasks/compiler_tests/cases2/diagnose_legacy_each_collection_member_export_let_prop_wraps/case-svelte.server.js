import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let item = $$props["item"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(item.list);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let entry = each_array[$$index];
			$$renderer.push(`<!---->${$.escape(entry)}`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { item });
	});
}
