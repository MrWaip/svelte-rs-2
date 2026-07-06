import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	let message = "one";
	function attachment(message) {
		return (node) => {
			node.textContent = message;
		};
	}
	Inner($$renderer, {});
}
