import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [{ a: 1 }];
	$$renderer.push(`<button>add</button> <!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let { a } = each_array[$$index];
		$$renderer.push(`<span>${$.escape(a)}</span>`);
	}
	$$renderer.push(`<!--]-->`);
}
