import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer) {
		$$renderer.push(`<!---->failed text`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<p>a</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
