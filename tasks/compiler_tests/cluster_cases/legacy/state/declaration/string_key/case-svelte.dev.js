import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = $.tag($.mutable_source(tmp["a-b"]), "ab"), cd = $.tag($.mutable_source(tmp["c d"]), "cd");
	function bump() {
		$.set(ab, $.get(ab));
		$.set(cd, $.get(cd));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
