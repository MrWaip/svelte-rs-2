import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, { $$slots: { x: ($$renderer) => {
		{
			$$renderer.push(`<p class="svelte-16b3ya8">hi</p>`);
		}
	} } });
}
