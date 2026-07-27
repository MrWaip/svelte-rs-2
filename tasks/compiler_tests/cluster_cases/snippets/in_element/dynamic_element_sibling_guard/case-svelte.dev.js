App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<b>hi</b>`), App[$.FILENAME], [[2, 30]]);
var root_1 = $.add_locations($.from_html(`<div><span> </span></div>`), App[$.FILENAME], [[
	2,
	0,
	[[2, 5]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var span = $.child(div);
	{
		const foo = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var b = root();
			$.append($$anchor, b);
		});
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $$props.x));
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
