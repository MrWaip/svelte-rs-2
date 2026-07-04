import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let arr = $.tag($.mutable_source([
		1,
		2,
		3
	]), "arr");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(arr), $.index, ($$anchor, o, $$index) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(arr)[$$index]));
		$.event("click", button, function click() {
			$.get(arr)[$$index] = $.get(arr)[$$index] * 2, $.invalidate_inner_signals(() => $.get(arr));
		});
		$.append($$anchor, button);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
