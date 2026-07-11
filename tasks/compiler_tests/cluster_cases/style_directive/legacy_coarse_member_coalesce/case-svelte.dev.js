import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.tag($.mutable_source(), "obj");
	function upd(e) {
		$.set(obj, e);
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { width: `${($.get(obj), $.untrack(() => ($.get(obj)?.w || 0) + 40)) ?? ""}px` }));
	$.event("click", div, upd);
	$.append($$anchor, div);
	return $.pop($$exports);
}
