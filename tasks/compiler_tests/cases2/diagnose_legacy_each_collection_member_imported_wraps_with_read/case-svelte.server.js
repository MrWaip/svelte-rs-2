import * as $ from "svelte/internal/server";
import { LINKS } from "./links.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let heading;
		let title = $.fallback($$props["title"], "");
		$: heading = title.toUpperCase();
		$$renderer.push(`<h1>${$.escape(heading)}</h1> <!--[-->`);
		const each_array = $.ensure_array_like(LINKS.list);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let link = each_array[$$index];
			$$renderer.push(`<a${$.attr("href", link.href)}>${$.escape(link.label)}</a>`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { title });
	});
}
