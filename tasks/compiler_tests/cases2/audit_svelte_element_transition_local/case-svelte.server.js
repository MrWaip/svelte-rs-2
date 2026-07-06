import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	let tag = "div";
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
		$.element($$renderer, tag, void 0, () => {
			$$renderer.push(`x`);
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
