App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Card from "./Card.svelte";
var root = $.add_locations($.from_html(`<p>Hello world</p>`), App[$.FILENAME], [[5, 6]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Card($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var p = root();
			$.append($$anchor, p);
		}),
		$$slots: { default: true }
	}), "component", App, 5, 0, { componentTag: "Card" });
	return $.pop($$exports);
}
