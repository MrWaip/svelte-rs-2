import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer, error) {
		$$renderer.push(`<p>outer: ${$.escape(error.message)}</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			function failed($$renderer, error) {
				$$renderer.push(`<p>${$.escape(error.message)}</p>`);
			}
			$$renderer.boundary({ failed }, ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				{
					$$renderer.push(`<p>inner</p>`);
				}
				$$renderer.push(`<!--]-->`);
			});
		}
		$$renderer.push(`<!--]-->`);
	});
}
