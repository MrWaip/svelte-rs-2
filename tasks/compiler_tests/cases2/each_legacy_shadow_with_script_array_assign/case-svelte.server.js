import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let items = $$props["items"];
	let item = $$props["item"];
	let target;
	function handle(e) {
		[target] = e;
	}
	$$renderer.push(`<!---->${$.escape(handle)} <!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<div>${$.escape(item.id)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		items,
		item
	});
}
