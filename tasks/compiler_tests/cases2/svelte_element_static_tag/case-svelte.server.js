import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.element($$renderer, "div", void 0, () => {
		$$renderer.push(`Hello`);
	});
}
