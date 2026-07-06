import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	let visible = true;
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div>hello</div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
