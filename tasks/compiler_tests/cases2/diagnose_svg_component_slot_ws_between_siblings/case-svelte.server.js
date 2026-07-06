import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`);
		},
		$$slots: { default: true }
	});
}
