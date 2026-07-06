import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = $$props["x"];
		if ("Eva".startsWith("E")) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`eee`);
		} else if (x) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`def`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`rrr`);
		}
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { x });
	});
}
