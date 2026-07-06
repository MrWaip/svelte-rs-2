import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let title = "hello";
	$.element($$renderer, tag, () => {
		$$renderer.push(` class="my-class"`);
	}, () => {
		$$renderer.push(`Content: hello`);
	});
}
