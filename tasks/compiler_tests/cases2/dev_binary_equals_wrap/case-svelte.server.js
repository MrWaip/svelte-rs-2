import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 1;
	let b = 2;
	if (a === b) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`equal`);
	} else if (a == 1) {
		$$renderer.push("<!--[1-->");
		$$renderer.push(`one`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> true
true
true`);
}
