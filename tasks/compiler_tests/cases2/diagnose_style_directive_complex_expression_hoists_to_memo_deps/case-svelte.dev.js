import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let c = $.prop($$props, "c", 8);
	function b(x) {
		return x;
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(($0) => styles = $.set_style(div, "", styles, $0), [() => ({ background: ($.deep_read_state(a()), $.deep_read_state(c()), $.untrack(() => a() || b(c()))) })]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
