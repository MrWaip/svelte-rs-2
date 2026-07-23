import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer) {
		$$renderer.push(`<!---->z`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			function a($$renderer) {
				$$renderer.push(`<!---->1`);
			}
			function b($$renderer) {
				$$renderer.push(`<!---->2`);
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
