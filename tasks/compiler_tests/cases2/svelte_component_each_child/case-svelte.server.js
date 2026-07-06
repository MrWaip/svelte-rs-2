import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	let current = A;
	let items = [
		1,
		2,
		3
	];
	if (current) {
		$$renderer.push("<!--[-->");
		current($$renderer, {
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				const each_array = $.ensure_array_like(items);
				for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
					let item = each_array[$$index];
					$$renderer.push(`<span>${$.escape(item)}</span>`);
				}
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
