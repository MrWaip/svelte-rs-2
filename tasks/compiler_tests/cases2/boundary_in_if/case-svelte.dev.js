App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 3]]);
var root_1 = $.add_locations($.from_html(`<p>guarded</p>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
					$.validate_snippet_args(...arguments);
					var p = root();
					var text = $.child(p, true);
					$.reset(p);
					$.template_effect(() => $.set_text(text, error().message));
					$.append($$anchor, p);
				});
				$.boundary(node_1, { failed }, ($$anchor) => {
					var p_1 = root_1();
					$.append($$anchor, p_1);
				});
			}
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
