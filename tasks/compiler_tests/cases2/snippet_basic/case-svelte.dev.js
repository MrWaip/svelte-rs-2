App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const greeting = $.wrap_snippet(App, function($$anchor, name = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${name() ?? ""}`));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = $.prop($$props, "title", 3, "world");
	let message = "hello";
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => greeting(node, () => message), "render", App, 10, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => greeting(node_1, title), "render", App, 11, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
