import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
var root = $.add_locations($.from_html(`<div slot="cell"> </div>`), App[$.FILENAME], [[6, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 24, () => []);
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { cell: ($$anchor, $$slotProps) => {
		const index = $.derived_safe_equal(() => $$slotProps.index);
		const style = $.derived_safe_equal(() => $$slotProps.style);
		const item = $.tag($.derived_safe_equal(() => ($.deep_read_state(items()), $.deep_read_state($.get(index)), $.untrack(() => items()[$.get(index)]))), "item");
		$.get(item);
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => {
			$.set_style(div, $.get(style));
			$.set_text(text, $.get(item));
		});
		$.append($$anchor, div);
	} } }), "component", App, 5, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
