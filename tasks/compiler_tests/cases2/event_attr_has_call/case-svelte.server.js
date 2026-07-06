import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function getHandler() {
		return () => console.log("clicked");
	}
	$$renderer.push(`<button>Click</button>`);
}
