import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = [{
		a: { b: "x" },
		key: "b"
	}];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(a);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { a, key } = each_array[$$index];
		$$renderer.push(`<input${$.attr("value", a[key])}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
