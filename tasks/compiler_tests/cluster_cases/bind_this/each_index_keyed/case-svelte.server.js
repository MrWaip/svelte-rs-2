import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let data = $.fallback($$props["data"], () => [{ id: "1" }], true);
		let refs = $.fallback($$props["refs"], () => [], true);
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(data);
		for (let index = 0, $$length = each_array.length; index < $$length; index++) {
			let { id } = each_array[index];
			$$renderer.push(`<div>${$.escape(id)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, {
			data,
			refs
		});
	});
}
