import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	const registry = { Widget };
	if (registry.Widget) {
		$$renderer.push("<!--[-->");
		registry.Widget($$renderer, {});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
