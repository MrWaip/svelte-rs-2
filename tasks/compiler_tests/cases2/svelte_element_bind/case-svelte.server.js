import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let el = void 0;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`content`);
	});
}
