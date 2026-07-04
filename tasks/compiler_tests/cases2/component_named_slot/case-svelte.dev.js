App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.add_locations($.from_html(`<p slot="footer">Footer</p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Widget($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var p = root();
		$.append($$anchor, p);
	} } }), "component", App, 5, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
