import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => List($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		const item = $.derived(() => {
			let { text } = $$slotProps.item;
			return { text };
		});
		var p = root();
		var text_1 = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text_1, $.get(item).text));
		$.append($$anchor, p);
	} } }), "component", App, 5, 0, { componentTag: "List" });
	return $.pop($$exports);
}
