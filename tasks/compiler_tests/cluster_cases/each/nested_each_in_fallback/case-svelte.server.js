import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = [];
	let b = [];
	const each_array = $.ensure_array_like(a);
	if (each_array.length !== 0) {
		$$renderer.push("<!--[-->");
		for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
			let x = each_array[$$index_1];
			$$renderer.push(`<p>${$.escape(x)}</p>`);
		}
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push(`<!--[-->`);
		const each_array_1 = $.ensure_array_like(b);
		for (let $$index = 0, $$length = each_array_1.length; $$index < $$length; $$index++) {
			let x = each_array_1[$$index];
			$$renderer.push(`<span>${$.escape(x)}</span>`);
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
}
