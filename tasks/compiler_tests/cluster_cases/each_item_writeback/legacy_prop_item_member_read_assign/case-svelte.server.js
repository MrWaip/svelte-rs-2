import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let arr = $.fallback($$props["arr"], () => [{ prop: "foo" }], true);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(arr);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let o = each_array[$$index];
		$$renderer.push(`<span>${$.escape(o.prop)}</span> <button>x</button>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { arr });
}
