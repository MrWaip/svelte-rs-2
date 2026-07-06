import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	{
		function extra_element($$renderer) {
			$$renderer.push(`<span>hi</span>`);
		}
		Child($$renderer, {
			extra_element,
			$$slots: { extra_element: true }
		});
	}
}
