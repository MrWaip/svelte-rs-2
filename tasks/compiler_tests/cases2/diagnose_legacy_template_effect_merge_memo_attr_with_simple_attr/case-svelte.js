import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8, 0);
	let label = $.prop($$props, "label", 8, "");
	function toPx(n) {
		return n + "px";
	}
	var div = root();
	$.template_effect(($0) => {
		$.set_style(div, `--w: ${$0 ?? ""};`);
		$.set_attribute(div, "data-testid", label());
	}, [() => ($.deep_read_state(value()), $.untrack(() => toPx(value())))]);
	$.append($$anchor, div);
}
