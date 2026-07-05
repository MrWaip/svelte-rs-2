import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let p = Promise.resolve({});
	let num = $.tag($.mutable_source(0), "num");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { v = $.get(num) } = $.get($$source);
			return { v };
		});
		var v = $.derived_safe_equal(() => $.get($$value).v);
		var button_1 = root();
		var text = $.child(button_1, true);
		$.reset(button_1);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, button_1);
	}), "await", App, 7, 0);
	$.event("click", button, function click() {
		return $.update(num);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
