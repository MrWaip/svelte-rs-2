import * as $ from "svelte/internal/server";
import { items } from "./data";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let foo = $$props["foo"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items());
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<div>${$.escape(item)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { foo });
	});
}
