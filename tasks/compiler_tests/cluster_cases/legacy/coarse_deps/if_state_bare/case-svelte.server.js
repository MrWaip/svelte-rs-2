import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let foo = 1;
	function inc() {
		foo = foo + 1;
	}
	$$renderer.push(`<button>+</button>`);
	if (foo) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`a`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
