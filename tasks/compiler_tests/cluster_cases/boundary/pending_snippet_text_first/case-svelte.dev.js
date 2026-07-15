App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>reset</button>`), App[$.FILENAME], [[6, 2]]);
var root_1 = $.add_locations($.from_html(`<p>a</p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("pending");
			$.append($$anchor, text);
		});
		const failed = $.wrap_snippet(App, function($$anchor, _ = $.noop, reset = $.noop) {
			$.validate_snippet_args(...arguments);
			var button = root();
			$.delegated("click", button, function(...$$args) {
				$.apply(reset, this, $$args, App, [6, 19]);
			});
			$.append($$anchor, button);
		});
		$.boundary(node, {
			pending,
			failed
		}, ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
