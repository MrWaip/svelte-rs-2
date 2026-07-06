import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		if ([1, 2].includes(1)) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`eee`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	});
}
