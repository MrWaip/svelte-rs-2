import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div class="wrap svelte-5iy3wu">`);
	Child($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<span><p class="svelte-5iy3wu">hi</p></span>`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!----></div>`);
}
