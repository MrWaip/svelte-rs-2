import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8, 1);
	function makeBar() {
		return 42;
	}
	let bar = $.prop($$props, "bar", 24, makeBar);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo() ?? ""}${bar() ?? ""}`));
	$.append($$anchor, p);
}
