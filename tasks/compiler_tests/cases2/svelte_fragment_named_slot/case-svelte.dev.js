import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.add_locations($.from_html(`<p>First</p> <p>Second</p>`, 1), App[$.FILENAME], [[7, 2], [8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Widget($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var fragment_1 = root();
		$.next(2);
		$.append($$anchor, fragment_1);
	} } }), "component", App, 5, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
