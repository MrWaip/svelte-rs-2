import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let items = [{
		id: 1,
		active: false
	}, {
		id: 2,
		active: true
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attr_class("", void 0, { "active": item.active })}`);
		}, () => {
			$$renderer.push(`${$.escape(item.id)}`);
		});
	}
	$$renderer.push(`<!--]-->`);
}
