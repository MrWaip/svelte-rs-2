import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="wrap svelte-1ktsoc1">`);
	Child($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<p class="svelte-1ktsoc1">hi</p>`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!----></div>`);
}
