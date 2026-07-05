import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="wrap svelte-5iy3wu">`);
	Child($$renderer, { $$slots: { x: ($$renderer) => {
		{
			$$renderer.push(`<p class="svelte-5iy3wu">hi</p>`);
		}
	} } });
	$$renderer.push(`<!----></div>`);
}
