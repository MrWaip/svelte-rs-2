import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	Foo($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const onClick = $.derived_safe_equal(() => $$slotProps.onClick);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(onClick)));
			$.append($$anchor, p);
		} }
	});
}
