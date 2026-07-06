import * as $ from "svelte/internal/server";
import Row from "./Row.svelte";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let row = each_array[$$index];
		$.css_props($$renderer, true, { "--tone": "red" }, () => {
			Row($$renderer, { $$slots: { label: ($$renderer) => {
				$$renderer.push(`<span slot="label">${$.escape(row.title)}</span>`);
			} } });
		});
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
