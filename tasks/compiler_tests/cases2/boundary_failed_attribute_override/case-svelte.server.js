import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$anchor, error) {
		console.log("attribute", error);
	}
	function failed($$renderer, error) {
		$$renderer.push(`<p>${$.escape(error.message)}</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<p>content</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
