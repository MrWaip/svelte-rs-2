import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer) {
		$$renderer.push(`<!---->z`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			function other($$renderer) {
				$$renderer.push(`<!---->x`);
			}
			function failed($$renderer) {
				$$renderer.push(`<!---->y`);
			}
			$$renderer.boundary({ failed }, ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				{}
				$$renderer.push(`<!--]-->`);
			});
		}
		$$renderer.push(`<!--]-->`);
	});
}
