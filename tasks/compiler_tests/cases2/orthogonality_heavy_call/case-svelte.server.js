import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function compute() {
		return 1;
	}
	$$renderer.push(`<p>${$.escape(compute())}</p>`);
}
