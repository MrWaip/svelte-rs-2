import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	function tooltip(node) {
		return { destroy() {} };
	}
	Inner($$anchor, {
		[$.attachment()]: tooltip,
		prop: "value"
	});
}
