App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Card from "./Card.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Card($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var text = $.text("footer");
		$.append($$anchor, text);
	} } }), "component", App, 5, 0, { componentTag: "Card" });
	return $.pop($$exports);
}
