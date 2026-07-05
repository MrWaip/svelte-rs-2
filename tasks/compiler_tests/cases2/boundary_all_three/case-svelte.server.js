import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handleError(error) {
		console.error(error);
	}
	function failed($$renderer, error) {
		$$renderer.push(`<p>${$.escape(error.message)}</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[!-->`);
		{
			$$renderer.push(`<p>loading...</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
