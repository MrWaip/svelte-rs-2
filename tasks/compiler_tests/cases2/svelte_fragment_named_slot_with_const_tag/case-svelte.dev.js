import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => List($$anchor, { $$slots: { row: ($$anchor, $$slotProps) => {
		const row = $.derived_safe_equal(() => $$slotProps.row);
		const v = $.tag($.derived_safe_equal(() => ($.deep_read_state($.get(row)), $.untrack(() => $.get(row).value * 2))), "v");
		$.get(v);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, p);
	} } }), "component", App, 5, 0, { componentTag: "List" });
	return $.pop($$exports);
}
