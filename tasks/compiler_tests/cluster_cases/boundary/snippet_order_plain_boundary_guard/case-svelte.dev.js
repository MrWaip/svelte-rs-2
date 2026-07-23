App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const other = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("x");
			$.append($$anchor, text);
		});
		$.boundary(node, {}, ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = $.wrap_snippet(App, function($$anchor) {
					$.validate_snippet_args(...arguments);
					$.next();
					var text_1 = $.text("y");
					$.append($$anchor, text_1);
				});
				$.boundary(node_1, { failed }, ($$anchor) => {});
			}
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
