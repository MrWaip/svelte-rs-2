import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`some text`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
