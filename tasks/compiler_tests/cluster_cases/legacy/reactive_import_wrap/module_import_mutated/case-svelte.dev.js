import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import foo from "./foo.js";
import * as $ from "svelte/internal/client";
foo.bar = "baz";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(foo), $.untrack(() => foo.bar))));
	$.append($$anchor, p);
	return $.pop($$exports);
}
