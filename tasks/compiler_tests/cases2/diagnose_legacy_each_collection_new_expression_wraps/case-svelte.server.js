import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(new Array(4).fill(null));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			$$renderer.push(`<span>${$.escape(i)}</span>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
