import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { registry } = $$props;
	if (registry.Widget) {
		$$renderer.push("<!--[-->");
		registry.Widget($$renderer, {});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
