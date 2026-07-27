import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	const arr = $.mutable_source([1, 2]);
	(($$value) => {
		var $$array = $.to_array($$value, 2);
		$.mutate(arr, $.get(arr)[0] = $$array[0]);
		$.mutate(arr, $.get(arr)[1] = $.fallback($$array[1], $.get(arr)));
	})([$.get(arr)[1], $.get(arr)[0]]);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(arr)));
	$.append($$anchor, text);
}
