import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: null };
	let src = {};
	$.element($$renderer, "button", void 0, () => {
		$$renderer.push(`go`);
	});
}
