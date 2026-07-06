import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	function attachment() {
		console.log("up");
	}
	let enabled = false;
	$$renderer.push(`<button></button> `);
	Inner($$renderer, {});
	$$renderer.push(`<!---->`);
}
