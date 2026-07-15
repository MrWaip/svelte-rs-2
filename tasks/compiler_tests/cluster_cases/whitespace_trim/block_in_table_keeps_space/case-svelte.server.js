import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rows } = $$props;
	$$renderer.push(`<table><tbody><!--[-->`);
	const each_array = $.ensure_array_like(rows);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let r = each_array[$$index];
		$$renderer.push(`<tr><td>${$.escape(r)}</td></tr> <tr><td>${$.escape(r)}</td></tr>`);
	}
	$$renderer.push(`<!--]--></tbody></table>`);
}
