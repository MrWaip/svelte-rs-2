App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Widget($$anchor, { $$events: {
		focus($$arg) {
			$.bubble_event.call(this, $$props, $$arg);
		},
		keydown($$arg) {
			$.bubble_event.call(this, $$props, $$arg);
		}
	} }), "component", App, 5, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
