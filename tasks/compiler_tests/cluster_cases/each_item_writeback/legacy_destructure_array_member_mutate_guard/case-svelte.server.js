import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rows = $$props["rows"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let [entry] = each_array[$$index];
		$$renderer.push(`<button>${$.escape(entry.count)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { rows });
}
