import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 8);
	function fn(v) {
		return v + 1;
	}
	var p = root();
	$.template_effect(($0) => $.set_attribute(p, "data-x", $0), [() => ($.deep_read_state(value()), $.untrack(() => fn(value())))]);
	$.append($$anchor, p);
}
