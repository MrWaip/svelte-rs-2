import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = null;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`content`);
	});
}
