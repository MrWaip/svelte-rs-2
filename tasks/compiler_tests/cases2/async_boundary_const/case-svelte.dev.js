import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 2]]);
var root_1 = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			$.validate_snippet_args(...arguments);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, p);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			const doubled = $.tag($.derived(() => x * 2), "doubled");
			$.get(doubled);
			var p_1 = root_1();
			p_1.textContent = $.get(doubled);
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
