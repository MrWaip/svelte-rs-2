import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	let message = "one";
	function attachment(message) {
		return (node) => {
			node.textContent = message;
		};
	}
	Inner($$anchor, { [$.attachment()]: ($$node) => (attachment(message) || $.noop)($$node) });
}
