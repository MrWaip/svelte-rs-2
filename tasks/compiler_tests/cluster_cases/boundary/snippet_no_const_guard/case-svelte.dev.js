App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>failed</p>`), App[$.FILENAME], [[9, 2]]);
var root_1 = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var p = root();
			$.append($$anchor, p);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			var div = root_1();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $$props.n));
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
