import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let pending = null;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("wait");
			$.append($$anchor, text);
		});
		$.boundary(node, {
			pending,
			pending
		}, ($$anchor) => {
			$.next();
			var text_1 = $.text("hi");
			$.append($$anchor, text_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
