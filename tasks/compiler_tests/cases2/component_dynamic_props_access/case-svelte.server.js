import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { Widget } = $$props;
	if (Widget) {
		$$renderer.push("<!--[-->");
		Widget($$renderer, {});
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
