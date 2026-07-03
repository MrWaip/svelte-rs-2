App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const greeting = $.wrap_snippet(App, function($$anchor, msg = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${msg() ?? ""}`));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	var $$exports = { ...$.legacy_api() };
	var p_1 = root_1();
	p_1.textContent = "world";
	$.append($$anchor, p_1);
	return $.pop($$exports);
}
