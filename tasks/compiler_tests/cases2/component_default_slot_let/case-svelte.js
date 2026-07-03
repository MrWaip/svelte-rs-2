import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	List($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const item = $.derived_safe_equal(() => $$slotProps.item);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p);
		} }
	});
}
