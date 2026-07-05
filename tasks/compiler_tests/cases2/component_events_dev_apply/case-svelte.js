import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor) {
	let count = $.state(0);
	function getHandler() {
		return () => $.update(count);
	}
	var event_handler = $.derived(getHandler);
	Widget($$anchor, { $$events: { done(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	} } });
}
