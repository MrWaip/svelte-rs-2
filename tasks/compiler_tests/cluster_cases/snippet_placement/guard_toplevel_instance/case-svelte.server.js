import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "x";
	function t($$renderer) {
		$$renderer.push(`<!---->x`);
	}
	t($$renderer);
}
