import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handleClick() {
		console.log("clicked");
	}
	$$renderer.push(`<button>Click me</button>`);
}
