import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`if text`);
	} else if (false) {
		$$renderer.push("<!--[1-->");
		$$renderer.push(`else if text`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`else text`);
	}
	$$renderer.push(`<!--]-->`);
}
