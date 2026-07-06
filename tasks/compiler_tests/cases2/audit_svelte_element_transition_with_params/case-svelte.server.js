import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	let tag = "div";
	let duration = 300;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`x`);
	});
}
