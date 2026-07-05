App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let numbers = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "numbers");
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	{
		const x = $.wrap_snippet(App, function($$anchor, n = $.noop) {
			$.validate_snippet_args(...arguments);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, n()));
			$.append($$anchor, p);
		});
		$.add_svelte_meta(() => $.each(div, 21, () => numbers, $.index, ($$anchor, n) => {
			$.add_svelte_meta(() => x($$anchor, () => $.get(n)), "render", App, 10, 2);
		}), "each", App, 9, 1);
		$.reset(div);
	}
	$.append($$anchor, div);
	return $.pop($$exports);
}
