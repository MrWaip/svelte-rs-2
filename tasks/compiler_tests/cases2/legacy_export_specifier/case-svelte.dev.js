import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8, 1);
	function makeBar() {
		return 42;
	}
	let bar = $.prop($$props, "bar", 24, makeBar);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo() ?? ""}${bar() ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
