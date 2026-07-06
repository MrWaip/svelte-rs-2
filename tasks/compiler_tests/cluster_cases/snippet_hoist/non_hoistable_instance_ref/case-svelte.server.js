import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "x";
	function foo($$renderer) {
		$$renderer.push(`<!---->x`);
	}
	foo($$renderer);
}
