App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor) {
			const foo = $.tag($.derived(() => "bar"), "foo");
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text();
			text.nodeValue = $.get(foo);
			$.append($$anchor, text);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			const foo = $.tag($.derived(() => "bar"), "foo");
			$.get(foo);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
