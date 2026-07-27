import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (c) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`AB`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
