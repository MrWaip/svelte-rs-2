import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const greeting = "hello";
	const items = [
		1,
		2,
		3
	];
	$$renderer.push(`<p>hello</p> <!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let i = each_array[$$index];
		$$renderer.push(`<span>${$.escape(i)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
}
