App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const view = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	var $$array = $.derived(() => $.to_array(($$arg0?.()).list, 1));
	var $$array_1 = $.derived(() => $.to_array($$array[0]));
	let name = $.derived_safe_equal(() => $.fallback(($$arg0?.()).nested.name, "fallback"));
	$.get(name);
	let first = () => $.get($$array_1)[0];
	first();
	let rest = () => $.get($$array_1).slice(1);
	rest();
	let tail = () => $.exclude_from_object($$arg0?.(), ["nested", "list"]);
	tail();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""} ${first() ?? ""} ${rest().length ?? ""} ${tail().meta.note ?? ""}`));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = $.tag_proxy($.proxy({
		nested: { name: "world" },
		list: [[
			10,
			20,
			30
		]],
		meta: { note: "ok" }
	}), "data");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => view($$anchor, () => data), "render", App, 13, 0);
	return $.pop($$exports);
}
