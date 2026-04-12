import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	List($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const processed = $.derived_safe_equal(() => $$slotProps.item);
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, ($.deep_read_state($.get(processed)), $.untrack(() => $.get(processed).text))));
			$.append($$anchor, p);
		} }
	});
}
