import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.element($$renderer, "svg", void 0, () => {
		$$renderer.push(`<circle r="5"></circle>`);
	});
}
