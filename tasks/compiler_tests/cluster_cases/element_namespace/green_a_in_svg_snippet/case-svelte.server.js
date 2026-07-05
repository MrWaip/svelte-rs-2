import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function s($$renderer) {
		$$renderer.push(`<a><text>Hello</text></a>`);
	}
	$$renderer.push(`<svg>`);
	s($$renderer);
	$$renderer.push(`<!----></svg>`);
}
