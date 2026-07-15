import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { items } = $$props;
	$$renderer.push(`<svg><!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let i = each_array[$$index];
		$$renderer.push(`<rect></rect><rect></rect>`);
	}
	$$renderer.push(`<!--]--></svg>`);
}
