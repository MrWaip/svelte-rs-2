import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	let x = false;
	let y = true;
	if (x) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>first</p>`);
	} else if (y) {
		$$renderer.push("<!--[1-->");
		$$renderer.push(`<div>second</div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
