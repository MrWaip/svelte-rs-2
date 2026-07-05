import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = { outer: [{ inner: 1 }] }, $$array = $.derived(() => $.to_array(tmp.outer, 1)), inner = $.tag($.mutable_source($.get($$array)[0].inner), "inner");
	function bump() {
		$.set(inner, $.get(inner));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(inner)));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
