App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const greeting = $.wrap_snippet(App, function($$anchor, msg = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${msg() ?? ""}`));
	$.append($$anchor, p);
});
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => greeting($$anchor, () => "world"), "render", App, 5, 0);
	return $.pop($$exports);
}
