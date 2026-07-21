import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	A($$anchor, {
		children: ($$anchor, $$slotProps) => {
			const foo = $.derived_safe_equal(() => $$slotProps.foo);
			var div = root();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $.get(foo)));
			$.append($$anchor, div);
		},
		$$slots: { default: true }
	});
}
