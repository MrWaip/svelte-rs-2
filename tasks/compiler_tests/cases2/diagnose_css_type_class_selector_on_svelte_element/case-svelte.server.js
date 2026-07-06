import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "button";
	$.element($$renderer, tag, () => {
		$$renderer.push(` class="x svelte-14nl1h2"`);
	}, () => {
		$$renderer.push(`hi`);
	});
}
