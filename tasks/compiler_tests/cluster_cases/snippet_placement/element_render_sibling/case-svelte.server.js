import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function t($$renderer) {
		$$renderer.push(`<span>hi</span>`);
	}
	$$renderer.push(`<div>`);
	t($$renderer);
	$$renderer.push(`<!----></div>`);
}
