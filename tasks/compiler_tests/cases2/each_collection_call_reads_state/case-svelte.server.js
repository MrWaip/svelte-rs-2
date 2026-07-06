import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $.fallback($$props["count"], 3);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(Array.from({ length: count }));
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let _ = each_array[$$index];
		$$renderer.push(`<span></span>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { count });
}
