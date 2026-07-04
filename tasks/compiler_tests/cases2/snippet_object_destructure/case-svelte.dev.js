App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const greeting = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let name = () => ($$arg0?.()).name;
	name();
	let age = () => ($$arg0?.()).age;
	age();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${name() ?? ""} is ${age() ?? ""}`));
	$.append($$anchor, p);
});
const withDefault = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let label = $.derived_safe_equal(() => $.fallback(($$arg0?.()).label, "default"));
	$.get(label);
	var span = root_1();
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, $.get(label)));
	$.append($$anchor, span);
});
const withRest = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let id = () => ($$arg0?.()).id;
	id();
	let rest = () => $.exclude_from_object($$arg0?.(), ["id"]);
	rest();
	var div = root_2();
	var text_2 = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text_2, id()));
	$.append($$anchor, div);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[10, 1]]);
var root_2 = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[14, 1]]);
var root_3 = $.add_locations($.from_html(`<!> <!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = $.tag_proxy($.proxy({
		name: "world",
		age: 25
	}), "data");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_3();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => greeting(node, () => data), "render", App, 17, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => withDefault(node_1, () => ({})), "render", App, 18, 0);
	var node_2 = $.sibling(node_1, 2);
	$.add_svelte_meta(() => withRest(node_2, () => ({
		id: 1,
		extra: true
	})), "render", App, 19, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
