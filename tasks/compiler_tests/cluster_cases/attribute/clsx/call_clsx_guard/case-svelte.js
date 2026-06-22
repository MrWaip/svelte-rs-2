import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	function fn(v) {
		return v;
	}
	var div = root();
	$.template_effect(($0) => $.set_class(div, 1, $0), [() => $.clsx(($.deep_read_state(x()), $.untrack(() => fn(x()))))]);
	$.append($$anchor, div);
}
