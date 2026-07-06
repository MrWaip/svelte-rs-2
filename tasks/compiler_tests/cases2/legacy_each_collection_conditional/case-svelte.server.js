import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let cond = $.fallback($$props["cond"], true);
	let a = $.fallback($$props["a"], () => [1], true);
	let b = $.fallback($$props["b"], () => [2], true);
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(cond ? a : b);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<span>${$.escape(item)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, {
		cond,
		a,
		b
	});
}
