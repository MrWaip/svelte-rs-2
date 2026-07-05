import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function pending($$anchor) {
		console.log($$anchor);
	}
	$$renderer.push(`<!--[!-->`);
	pending($$renderer);
	$$renderer.push(`<!--]-->`);
}
