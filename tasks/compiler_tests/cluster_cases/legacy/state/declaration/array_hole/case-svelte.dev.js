import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp, 3)), a = $.tag($.mutable_source($.get($$array)[0]), "a"), c = $.tag($.mutable_source($.get($$array)[2]), "c");
	function bump() {
		$.set(a, $.get(a));
		$.set(c, $.get(c));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(c) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
