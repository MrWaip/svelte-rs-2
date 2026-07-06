import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	function failed($$renderer, error) {
		$$renderer.push(`<p>${$.escape(error.message)}</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			const doubled = x * 2;
			$$renderer.push(`<p>2</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
