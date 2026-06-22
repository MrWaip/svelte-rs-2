import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import foo from "./foo.js";
var $$_import_foo = $.reactive_import(() => foo);
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$$_import_foo($$_import_foo().bar = "baz");
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state($$_import_foo()), $.untrack(() => $$_import_foo().bar))));
	$.append($$anchor, p);
	$.pop();
}
