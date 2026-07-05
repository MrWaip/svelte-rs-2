import * as $ from "svelte/internal/server";
import { slide } from "svelte/transition";
export default function App($$renderer) {
	let visible = true;
	function k() {}
	function c() {}
	if (visible) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div>hi</div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
