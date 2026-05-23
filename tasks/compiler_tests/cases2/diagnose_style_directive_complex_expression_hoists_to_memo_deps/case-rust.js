import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8);
	let c = $.prop($$props, "c", 8);
	function b(x) {
		return x;
	}
	var div = root();
	let styles;
	$.template_effect(($0) => styles = $.set_style(div, "", styles, $0), [() => ({ background: ($.deep_read_state(a()), $.deep_read_state(c()), $.untrack(() => a() || b(c()))) })]);
	$.append($$anchor, div);
}
