import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let name = $$props["name"];
		if ("abc".startsWith(name)) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`eee`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { name });
	});
}
