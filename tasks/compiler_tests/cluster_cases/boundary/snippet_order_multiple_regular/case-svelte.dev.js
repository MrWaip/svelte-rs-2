App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const a = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("1");
			$.append($$anchor, text);
		});
		const b = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text_1 = $.text("2");
			$.append($$anchor, text_1);
		});
		const failed = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text_2 = $.text("z");
			$.append($$anchor, text_2);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = $.wrap_snippet(App, function($$anchor) {
					$.validate_snippet_args(...arguments);
					$.next();
					var text_3 = $.text("y");
					$.append($$anchor, text_3);
				});
				$.boundary(node_1, { failed }, ($$anchor) => {});
			}
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
