import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		if ("Eva".startsWith("E")) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`eee`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`rrr`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
