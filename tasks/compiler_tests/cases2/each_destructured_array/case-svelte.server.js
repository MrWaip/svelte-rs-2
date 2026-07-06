import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { pairs = [] } = $$props;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(pairs);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let [key, val] = each_array[$$index];
		$$renderer.push(`<p>${$.escape(key)}=${$.escape(val)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
