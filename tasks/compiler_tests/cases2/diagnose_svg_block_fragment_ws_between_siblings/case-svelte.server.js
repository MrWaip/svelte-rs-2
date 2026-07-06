import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (cond) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
