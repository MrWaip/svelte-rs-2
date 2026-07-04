App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const defaultWrapWith = $.wrap_snippet(App, function($$anchor, mf = $.noop) {
	$.validate_snippet_args(...arguments);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.snippet(node, mf), "render", App, 7, 4);
	$.append($$anchor, fragment);
});
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[11, 4]]);
var root_1 = $.add_locations($.from_html(`<style>:root { --x: red; }</style>`), App[$.FILENAME], [[17, 4]]);
var root_2 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const inner = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${label() ?? ""}0`));
		$.append($$anchor, span);
	});
	let wrapWith = $.prop($$props, "wrapWith", 3, defaultWrapWith), label = $.prop($$props, "label", 3, "");
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	var div = root_2();
	$.head("q2w0q4", ($$anchor) => {
		var style = root_1();
		$.append($$anchor, style);
	});
	var node_1 = $.child(div);
	$.add_svelte_meta(() => $.snippet(node_1, wrapWith, () => inner), "render", App, 14, 5);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
