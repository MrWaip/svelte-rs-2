import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let rows = [];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let _ = 0, $$length = each_array.length; _ < $$length; _++) {
		let row = each_array[_];
		$$renderer.push(`<p>${$.escape(row.name)}</p>`);
	}
	$$renderer.push(`<!--]--> <!--[-->`);
	const each_array_1 = $.ensure_array_like(rows);
	for (let i_dx = 0, $$length = each_array_1.length; i_dx < $$length; i_dx++) {
		let row = each_array_1[i_dx];
		$$renderer.push(`<span>${$.escape(row.name)}${$.escape(i_dx)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
}
