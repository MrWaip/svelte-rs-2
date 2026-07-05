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
	], $$array = $.derived(() => $.to_array(tmp)), a = $.tag($.mutable_source($.get($$array)[0]), "a"), rest = $.get($$array).slice(1);
	function bump() {
		$.set(a, $.get(a));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.untrack(() => rest.length) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
