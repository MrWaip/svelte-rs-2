import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let arr = $.tag($.mutable_source([1, 2]), "arr");
	function swap() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.mutate(arr, $.get(arr)[0] = $$array[0]);
			$.mutate(arr, $.get(arr)[1] = $$array[1]);
		})([$.get(arr)[1], $.get(arr)[0]]);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${($.get(arr), $.untrack(() => $.get(arr)[0])) ?? ""}${($.get(arr), $.untrack(() => $.get(arr)[1])) ?? ""}`));
	$.event("click", button, swap);
	$.append($$anchor, button);
	return $.pop($$exports);
}
