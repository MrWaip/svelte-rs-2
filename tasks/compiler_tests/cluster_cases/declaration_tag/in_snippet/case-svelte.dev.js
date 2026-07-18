App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor, item = $.noop) {
	$.validate_snippet_args(...arguments);
	const label = item().name;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, label));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[3, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => row($$anchor, () => ({ name: "x" })), "render", App, 5, 0);
	return $.pop($$exports);
}
