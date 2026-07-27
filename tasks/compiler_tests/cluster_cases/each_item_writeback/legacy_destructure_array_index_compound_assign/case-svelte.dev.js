import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 1));
		let first = () => $.get($$array)[0];
		first();
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, first()));
		$.event("click", button, function click() {
			return $$array[0] = first() + 2, $.invalidate_inner_signals(() => rows());
		});
		$.append($$anchor, button);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
