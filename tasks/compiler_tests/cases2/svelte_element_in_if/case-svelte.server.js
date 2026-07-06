import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = true;
	let tag = "div";
	if (show) {
		$$renderer.push("<!--[0-->");
		$.element($$renderer, tag, void 0, () => {
			$$renderer.push(`content`);
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
