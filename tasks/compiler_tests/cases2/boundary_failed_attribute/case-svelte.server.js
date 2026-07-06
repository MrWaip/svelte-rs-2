import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function myFailed($$anchor, error) {
		console.log(error);
	}
	$$renderer.boundary({ failed: myFailed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<p>content</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
