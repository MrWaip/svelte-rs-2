import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let shown = true;
	$$renderer.push(`<svg>`);
	if (shown) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<title>Chart</title>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></svg>`);
}
