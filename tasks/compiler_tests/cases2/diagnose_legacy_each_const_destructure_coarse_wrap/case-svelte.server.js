import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	const lookup = {
		a: {
			x: 1,
			y: 2
		},
		b: {
			x: 3,
			y: 4
		}
	};
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let row = each_array[$$index];
		const { x, y } = lookup[row.key];
		$$renderer.push(`<p>${$.escape(x)}:${$.escape(y)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
