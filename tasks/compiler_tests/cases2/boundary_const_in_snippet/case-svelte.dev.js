App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 2]]);
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor, error = $.noop) {
			const x = $.tag($.derived(() => items.length), "x");
			$.validate_snippet_args(...arguments);
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}: ${error().message ?? ""}`));
			$.append($$anchor, p);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			const x = $.tag($.derived(() => items.length), "x");
			$.get(x);
			var p_1 = root_1();
			var text_1 = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text_1, $.get(x)));
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
