import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const bubbler = createBubbler();
		$$renderer.push(`<input type="text"/>`);
	});
}
