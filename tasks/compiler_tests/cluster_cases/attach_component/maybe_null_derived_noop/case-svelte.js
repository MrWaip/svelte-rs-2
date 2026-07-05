import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	let handler = $.derived(() => $$props.maybe ? (node) => {} : null);
	Inner($$anchor, { [$.attachment()]: ($$node) => ($.get(handler) || $.noop)($$node) });
}
