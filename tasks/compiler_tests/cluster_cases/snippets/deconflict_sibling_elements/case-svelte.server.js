import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function row($$renderer, n) {
		$$renderer.push(`<span>${$.escape(n)}</span>`);
	}
	function row($$renderer, n) {
		$$renderer.push(`<b>${$.escape(n)}</b>`);
	}
	$$renderer.push(`<div>`);
	row($$renderer, 1);
	$$renderer.push(`<!----></div> <div>`);
	row($$renderer, 2);
	$$renderer.push(`<!----></div>`);
}
