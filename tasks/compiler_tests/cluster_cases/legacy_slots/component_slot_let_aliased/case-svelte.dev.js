import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Nested from "./Nested.svelte";
import SlotInner from "./SlotInner.svelte";
var root = $.add_locations($.from_html(`<div class="inner-slot"> </div>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Nested($$anchor, { $$slots: { foo: ($$anchor, $$slotProps) => {
		const data = $.derived_safe_equal(() => $$slotProps.thing);
		$.add_svelte_meta(() => SlotInner($$anchor, {
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
		}), "component", App, 7, 1, { componentTag: "SlotInner" });
	} } }), "component", App, 6, 0, { componentTag: "Nested" });
	return $.pop($$exports);
}
