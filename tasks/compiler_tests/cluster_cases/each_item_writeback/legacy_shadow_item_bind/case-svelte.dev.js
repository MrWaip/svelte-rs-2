import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <input/>`, 1), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(["Hello"]), "a");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(a), $.index, ($$anchor, a, $$index, $$array) => {
		$.next();
		var fragment_1 = root();
		var text = $.first_child(fragment_1);
		var input = $.sibling(text);
		$.remove_input_defaults(input);
		$.template_effect(() => $.set_text(text, `${$$array()[$$index] ?? ""} `));
		$.bind_value(input, function get() {
			return $$array()[$$index];
		}, function set($$value) {
			$$array()[$$index] = $$value, $.invalidate_inner_signals(() => $$array());
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
