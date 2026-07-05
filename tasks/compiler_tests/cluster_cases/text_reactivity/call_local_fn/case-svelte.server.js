import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function fn() {
		return 1;
	}
	$$renderer.push(`<p>v ${$.escape(fn())}</p>`);
}
