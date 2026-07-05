import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	const registry_name = { Widget };
	if (registry_name.Widget) {
		$$renderer.push("<!--[-->");
		registry_name.Widget($$renderer, {});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
