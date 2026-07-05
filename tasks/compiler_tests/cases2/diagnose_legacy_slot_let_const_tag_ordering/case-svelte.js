import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
var root = $.from_html(`<div slot="cell"> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let items = $.prop($$props, "items", 24, () => []);
	$.init();
	Outer($$anchor, { $$slots: { cell: ($$anchor, $$slotProps) => {
		const index = $.derived_safe_equal(() => $$slotProps.index);
		const style = $.derived_safe_equal(() => $$slotProps.style);
		const item = $.derived_safe_equal(() => ($.deep_read_state(items()), $.deep_read_state($.get(index)), $.untrack(() => items()[$.get(index)])));
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => {
			$.set_style(div, $.get(style));
			$.set_text(text, $.get(item));
		});
		$.append($$anchor, div);
	} } });
	$.pop();
}
