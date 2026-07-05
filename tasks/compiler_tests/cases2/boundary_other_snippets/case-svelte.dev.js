App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>helper text</span>`), App[$.FILENAME], [[5, 2]]);
var root_1 = $.add_locations($.from_html(`<p> </p> <!>`, 1), App[$.FILENAME], [[9, 2]]);
var root_2 = $.add_locations($.from_html(`<p>content</p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const helper = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var span = root();
			$.append($$anchor, span);
		});
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			$.validate_snippet_args(...arguments);
			var fragment_1 = root_1();
			var p = $.first_child(fragment_1);
			var text = $.child(p, true);
			$.reset(p);
			var node_1 = $.sibling(p, 2);
			$.add_svelte_meta(() => helper(node_1), "render", App, 10, 2);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, fragment_1);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			var p_1 = root_2();
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
