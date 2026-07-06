import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="wrap svelte-1pc7tr6">`);
	Child($$renderer, { $$slots: { x: ($$renderer) => {
		{
			$$renderer.push(`<div class="mid svelte-1pc7tr6"><p class="svelte-1pc7tr6">hi</p></div>`);
		}
	} } });
	$$renderer.push(`<!----></div>`);
}
