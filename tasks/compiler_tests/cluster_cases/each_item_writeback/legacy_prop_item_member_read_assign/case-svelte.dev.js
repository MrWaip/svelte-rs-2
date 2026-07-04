import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span> <button>x</button>`, 1), App[$.FILENAME], [[7, 1], [8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let arr = $.prop($$props, "arr", 24, () => [{ prop: "foo" }]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, arr, $.index, ($$anchor, o, $$index) => {
		var fragment_1 = root();
		var span = $.first_child(fragment_1);
		var text = $.child(span, true);
		$.reset(span);
		var button = $.sibling(span, 2);
		$.template_effect(() => $.set_text(text, (arr()[$$index], $.untrack(() => arr()[$$index].prop))));
		$.event("click", button, function click() {
			return arr()[$$index] = {
				...arr()[$$index],
				prop: "bar"
			}, $.invalidate_inner_signals(() => arr());
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
