import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const k = "z";
	let tmp = { z: 1 }, v = $.tag($.mutable_source(tmp[k]), "v");
	function bump() {
		$.set(v, $.get(v));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(v)));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
