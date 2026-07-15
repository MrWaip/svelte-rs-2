import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Nested from "./Nested.svelte";
import SlotInner from "./SlotInner.svelte";
var root = $.from_html(`<div class="inner-slot"> </div>`);
export default function App($$anchor) {
	Nested($$anchor, { $$slots: { foo: ($$anchor, $$slotProps) => {
		const data = $.derived_safe_equal(() => $$slotProps.thing);
		SlotInner($$anchor, {
			slot: "foo",
			get thing() {
				return $.get(data);
			},
			children: $.invalid_default_snippet,
			$$slots: { default: ($$anchor, $$slotProps) => {
				var div = root();
				var text = $.child(div, true);
				$.reset(div);
				$.template_effect(() => $.set_text(text, $.get(data)));
				$.append($$anchor, div);
			} }
		});
	} } });
}
