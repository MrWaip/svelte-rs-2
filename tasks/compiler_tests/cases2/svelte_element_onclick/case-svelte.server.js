import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "button";
	let count = 0;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`Click`);
	});
}
