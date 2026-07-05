App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="result"><h1>Result</h1> <p> </p></div>`), App[$.FILENAME], [[
	10,
	1,
	[[11, 2], [12, 2]]
]]);
var root_1 = $.add_locations($.from_html(`<div class="error"><h1>Error</h1> <p> </p></div>`), App[$.FILENAME], [[
	15,
	1,
	[[16, 2], [17, 2]]
]]);
var root_2 = $.add_locations($.from_html(`<div class="loading"><span>Please wait...</span></div>`), App[$.FILENAME], [[
	6,
	1,
	[[7, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const promise = fetch("/api");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => promise, ($$anchor) => {
		var div_2 = root_2();
		$.append($$anchor, div_2);
	}, ($$anchor, value) => {
		var div = root();
		var p = $.sibling($.child(div), 2);
		var text = $.child(p, true);
		$.reset(p);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, div);
	}, ($$anchor, error) => {
		var div_1 = root_1();
		var p_1 = $.sibling($.child(div_1), 2);
		var text_1 = $.child(p_1, true);
		$.reset(p_1);
		$.reset(div_1);
		$.template_effect(() => $.set_text(text_1, $.get(error).message));
		$.append($$anchor, div_1);
	}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
