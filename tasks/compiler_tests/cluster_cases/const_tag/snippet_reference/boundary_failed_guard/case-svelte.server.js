import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer) {
		$$renderer.push(`<!---->bar`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			const foo = "bar";
		}
		$$renderer.push(`<!--]-->`);
	});
}
