App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[2, 18]]);
var root_1 = $.add_locations($.from_html(`<b> </b>`), App[$.FILENAME], [[6, 18]]);
var root_2 = $.add_locations($.from_html(`<div><!></div> <div><!></div>`, 1), App[$.FILENAME], [[1, 0], [5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var div = $.first_child(fragment);
	{
		const row = $.wrap_snippet(App, function($$anchor, n = $.noop) {
			$.validate_snippet_args(...arguments);
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, n()));
			$.append($$anchor, span);
		});
		var node = $.child(div);
		$.add_svelte_meta(() => row(node, () => 1), "render", App, 3, 1);
		$.reset(div);
	}
	var div_1 = $.sibling(div, 2);
	{
		const row = $.wrap_snippet(App, function($$anchor, n = $.noop) {
			$.validate_snippet_args(...arguments);
			var b = root_1();
			var text_1 = $.child(b, true);
			$.reset(b);
			$.template_effect(() => $.set_text(text_1, n()));
			$.append($$anchor, b);
		});
		var node_1 = $.child(div_1);
		$.add_svelte_meta(() => row(node_1, () => 2), "render", App, 7, 1);
		$.reset(div_1);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
