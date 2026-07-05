App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const badge = $.wrap_snippet(App, function($$anchor, text = $.noop) {
	$.validate_snippet_args(...arguments);
	var span = root_1();
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, text()));
	$.append($$anchor, span);
});
var root = $.add_locations($.from_html(`<meta name="description" content="test"/>`), App[$.FILENAME], [[6, 4]]);
var root_1 = $.add_locations($.from_html(`<span class="badge"> </span>`), App[$.FILENAME], [[10, 4]]);
var root_2 = $.add_locations($.from_html(`<div><p></p> <!></div>`), App[$.FILENAME], [[
	13,
	0,
	[[14, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = "hello";
	var $$exports = { ...$.legacy_api() };
	var div = root_2();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.append($$anchor, meta);
	});
	var p = $.child(div);
	p.textContent = "hello";
	var node = $.sibling(p, 2);
	$.add_svelte_meta(() => badge(node, () => "new"), "render", App, 15, 4);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
