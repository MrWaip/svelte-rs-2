import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.await($$renderer, items, () => {}, (result) => {
		$$renderer.push(`<ul><!--[-->`);
		const each_array = $.ensure_array_like(result);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<li>${$.escape(item)}</li>`);
		}
		$$renderer.push(`<!--]--></ul>`);
	});
	$$renderer.push(`<!--]-->`);
}
