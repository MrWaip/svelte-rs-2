import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="wrap svelte-1u3aiak">`);
	Child($$renderer, { $$slots: { x: ($$renderer) => {
		{
			$$renderer.push(`<p class="svelte-1u3aiak">hi</p>`);
		}
	} } });
	$$renderer.push(`<!----></div>`);
}
