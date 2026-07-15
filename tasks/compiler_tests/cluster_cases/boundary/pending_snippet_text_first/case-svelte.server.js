import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer, _, reset) {
		$$renderer.push(`<button>reset</button>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[!-->`);
		{
			$$renderer.push(`<!---->pending`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
