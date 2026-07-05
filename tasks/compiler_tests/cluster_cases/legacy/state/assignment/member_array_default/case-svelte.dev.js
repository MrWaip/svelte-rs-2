import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const a = 100;
	const arr = $.tag($.mutable_source([{ a: 1 }, 2]), "arr");
	function go() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.mutate(arr, $.get(arr)[0].a = $$array[0]);
			$.mutate(arr, $.get(arr)[1] = $.fallback($$array[1], a));
		})([$.get(arr)[1]]);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, $0), [() => ($.get(arr), $.untrack(() => JSON.stringify($.get(arr))))]);
	$.event("click", button, go);
	$.append($$anchor, button);
	return $.pop($$exports);
}
