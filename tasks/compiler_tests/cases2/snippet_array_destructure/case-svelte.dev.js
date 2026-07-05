App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const show = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	var $$array = $.derived(() => $.to_array($$arg0?.(), 2));
	let a = () => $.get($$array)[0];
	a();
	let b = () => $.get($$array)[1];
	b();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""} and ${b() ?? ""}`));
	$.append($$anchor, p);
});
const withRest = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	var $$array_1 = $.derived(() => $.to_array($$arg0?.()));
	let first = () => $.get($$array_1)[0];
	first();
	let others = () => $.get($$array_1).slice(1);
	others();
	var p_1 = root_1();
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => $.set_text(text_1, first()));
	$.append($$anchor, p_1);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 1]]);
var root_2 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let pair = $.tag_proxy($.proxy([10, 20]), "pair");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => show(node, () => pair), "render", App, 13, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => withRest(node_1, () => [
		1,
		2,
		3
	]), "render", App, 14, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
