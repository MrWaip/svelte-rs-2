import * as $ from "svelte/internal/server";
import { fade, fly } from "svelte/transition";
export default function App($$renderer) {
	let tag = "div";
	let show = true;
	let duration = 300;
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
