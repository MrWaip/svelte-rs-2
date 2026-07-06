import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let shown = true;
	$$renderer.push(`<foreignObject>`);
	if (shown) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div>fallback html</div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></foreignObject>`);
}
