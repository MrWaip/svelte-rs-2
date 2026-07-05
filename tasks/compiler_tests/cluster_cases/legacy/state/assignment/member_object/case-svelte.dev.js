import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.tag($.mutable_source({ x: 0 }), "obj");
	let src = { v: 1 };
	function go() {
		$.mutate(obj, $.get(obj).x = src.v);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(obj), $.untrack(() => $.get(obj).x))));
	$.event("click", button, go);
	$.append($$anchor, button);
	return $.pop($$exports);
}
