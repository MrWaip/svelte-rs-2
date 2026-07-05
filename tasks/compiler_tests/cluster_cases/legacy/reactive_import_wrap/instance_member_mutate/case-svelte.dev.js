import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import foo from "./foo.js";
var $$_import_foo = $.reactive_import(() => foo);
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	$$_import_foo($$_import_foo().bar = "baz");
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state($$_import_foo()), $.untrack(() => $$_import_foo().bar))));
	$.append($$anchor, p);
	return $.pop($$exports);
}
