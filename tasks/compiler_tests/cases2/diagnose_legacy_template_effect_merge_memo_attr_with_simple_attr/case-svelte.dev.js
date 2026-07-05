import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8, 0);
	let label = $.prop($$props, "label", 8, "");
	function toPx(n) {
		return n + "px";
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(($0) => {
		$.set_style(div, `--w: ${$0 ?? ""};`);
		$.set_attribute(div, "data-testid", label());
	}, [() => ($.deep_read_state(value()), $.untrack(() => toPx(value())))]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
