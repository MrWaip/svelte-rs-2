import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (Math.max(1, 2) > 1) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`eee`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
