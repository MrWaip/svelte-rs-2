App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>failed</p>`), App[$.FILENAME], [[13, 2]]);
var root_1 = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function compute() {
		return $$props.n + 1;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor) {
			const value = $.tag($.derived(compute), "value");
			$.validate_snippet_args(...arguments);
			var p = root();
			$.append($$anchor, p);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			const value = $.tag($.derived(compute), "value");
			$.get(value);
			var div = root_1();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, $.get(value)));
			$.append($$anchor, div);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
