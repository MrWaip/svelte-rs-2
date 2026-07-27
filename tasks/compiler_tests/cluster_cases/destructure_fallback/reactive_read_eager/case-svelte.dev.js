import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const arr = $.tag($.mutable_source([1, 2]), "arr");
	(($$value) => {
		var $$array = $.to_array($$value, 2);
		$.mutate(arr, $.get(arr)[0] = $$array[0]);
		$.mutate(arr, $.get(arr)[1] = $.fallback($$array[1], $.get(arr)));
	})([$.get(arr)[1], $.get(arr)[0]]);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(arr)));
	$.append($$anchor, text);
	return $.pop($$exports);
}
