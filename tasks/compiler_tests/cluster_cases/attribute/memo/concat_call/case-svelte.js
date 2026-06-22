import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let status = $.prop($$props, "status", 8, "neutral");
	function classify(s) {
		return s + "-x";
	}
	var div = root();
	$.template_effect(($0) => $.set_class(div, 1, `slider ${$0 ?? ""}`), [() => ($.deep_read_state(status()), $.untrack(() => classify(status()) || ""))]);
	$.append($$anchor, div);
}
