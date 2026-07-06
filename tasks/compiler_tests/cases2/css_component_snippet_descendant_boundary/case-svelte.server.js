import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="host svelte-1v67kh2">`);
	{
		function children($$renderer) {
			$$renderer.push(`<span class="inside svelte-1v67kh2">inside</span>`);
		}
		Widget($$renderer, {
			children,
			$$slots: { default: true }
		});
	}
	$$renderer.push(`<!----></div> <span class="inside">outside</span>`);
}
