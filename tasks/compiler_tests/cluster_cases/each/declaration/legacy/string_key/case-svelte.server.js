import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{
		"a-b": 1,
		"c d": 2
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { "a-b": ab, "c d": cd } = each_array[$$index];
		$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
