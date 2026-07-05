import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function foo() {
		return true;
	}
	if (foo()) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`eee`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
