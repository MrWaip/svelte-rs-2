import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
import { slide } from "svelte/transition";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const bubbler = createBubbler();
		$$renderer.push(`<div></div>`);
	});
}
