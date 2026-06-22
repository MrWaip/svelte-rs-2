import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let status = $.prop($$props, "status", 8, "neutral");
	let disabled = $.prop($$props, "disabled", 8, false);
	function classify(s) {
		return s + "-x";
	}
	function widthOf(s) {
		return s.length;
	}
	var div = root();
	let classes;
	$.template_effect(($0, $1) => {
		classes = $.set_class(div, 1, `slider ${$0 ?? ""}`, null, classes, { disabled: disabled() });
		$.set_style(div, `width: ${$1 ?? ""}px`);
	}, [() => ($.deep_read_state(status()), $.untrack(() => classify(status()) || "")), () => ($.deep_read_state(status()), $.untrack(() => widthOf(status())))]);
	$.append($$anchor, div);
}
