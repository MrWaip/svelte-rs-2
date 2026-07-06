import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const items1 = {};
	let data = [{
		id: 1,
		text: "a"
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(data);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<div>${$.escape(item.text)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}
