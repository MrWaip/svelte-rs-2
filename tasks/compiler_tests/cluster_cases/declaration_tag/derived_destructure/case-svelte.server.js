import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		let a = $.derived(() => item.a), b = $.derived(() => item.b);
		$$renderer.push(`<p>${$.escape(a())} ${$.escape(b())}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}
