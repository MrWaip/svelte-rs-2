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
			$$renderer.push(`<!--[!-->`);
			{
				$$renderer.push(`<!---->p`);
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
