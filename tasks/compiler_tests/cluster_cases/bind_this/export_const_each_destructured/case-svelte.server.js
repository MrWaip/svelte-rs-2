import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const items1 = {};
	let data = [{
		id: 1,
		text: "a"
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(data);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { id, text } = each_array[$$index];
		$$renderer.push(`<div>${$.escape(text)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { items1 });
}
