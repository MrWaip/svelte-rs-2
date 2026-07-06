import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let field = $$props["field"];
	let items = [{}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { [field]: value } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(value)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { field });
}
