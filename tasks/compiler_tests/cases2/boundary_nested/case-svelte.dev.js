App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[11, 2]]);
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 3]]);
var root_2 = $.add_locations($.from_html(`<p>inner</p>`), App[$.FILENAME], [[3, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			$.validate_snippet_args(...arguments);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `outer: ${error().message ?? ""}`));
			$.append($$anchor, p);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
					$.validate_snippet_args(...arguments);
					var p_1 = root_1();
					var text_1 = $.child(p_1, true);
					$.reset(p_1);
					$.template_effect(() => $.set_text(text_1, error().message));
					$.append($$anchor, p_1);
				});
				$.boundary(node_1, { failed }, ($$anchor) => {
					var p_2 = root_2();
					$.append($$anchor, p_2);
				});
			}
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
