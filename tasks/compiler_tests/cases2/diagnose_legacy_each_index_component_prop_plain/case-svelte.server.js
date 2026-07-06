import * as $ from "svelte/internal/server";
import Row from "./Row.svelte";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let index = 0, $$length = each_array.length; index < $$length; index++) {
		let row = each_array[index];
		Row($$renderer, {
			icon: index + 1,
			title: row.title
		});
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
