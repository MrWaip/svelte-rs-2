import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<span slot="default"> </span>`);
export default function App($$anchor) {
	A($$anchor, {
		children: ($$anchor, $$slotProps) => {
			const foo = $.derived_safe_equal(() => $$slotProps.foo);
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, $.get(foo)));
			$.append($$anchor, span);
		},
		$$slots: { default: true }
	});
}
