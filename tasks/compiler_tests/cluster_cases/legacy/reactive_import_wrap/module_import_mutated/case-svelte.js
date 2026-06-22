import "svelte/internal/flags/legacy";
import foo from "./foo.js";
import * as $ from "svelte/internal/client";
foo.bar = "baz";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(foo), $.untrack(() => foo.bar))));
	$.append($$anchor, p);
	$.pop();
}
