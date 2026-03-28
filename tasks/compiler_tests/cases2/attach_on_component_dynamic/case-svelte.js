import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor) {
	let enabled = true;
	let handler = $.derived(() => enabled ? (node) => {} : null);
	Inner($$anchor, {
		[$.attachment()]: ($$node) => $.get(handler)($$node),
		prop: "value"
	});
}
