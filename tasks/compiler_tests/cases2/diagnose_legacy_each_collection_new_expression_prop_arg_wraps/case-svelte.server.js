import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let size = $$props["size"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(new Array(size).fill(null));
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let _ = each_array[i];
			$$renderer.push(`<div>${$.escape(i)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { size });
	});
}
