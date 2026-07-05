import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => List($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const processed = $.derived_safe_equal(() => $$slotProps.item);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, ($.deep_read_state($.get(processed)), $.untrack(() => $.get(processed).text))));
			$.append($$anchor, p);
		} }
	}), "component", App, 5, 0, { componentTag: "List" });
	return $.pop($$exports);
}
