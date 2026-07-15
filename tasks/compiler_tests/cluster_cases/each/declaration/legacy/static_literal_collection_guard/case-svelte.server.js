import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like([
		1,
		2,
		3
	]);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let n = each_array[$$index];
		$$renderer.push(`<span>${$.escape(n)}${$.escape(x)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { x });
}
