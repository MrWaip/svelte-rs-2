import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	let refs = {};
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let row = each_array[$$index];
		$$renderer.push(`<div></div>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
