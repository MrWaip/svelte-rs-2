import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	function greet() {
		return "hi";
	}
	var $$exports = { greet };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.untrack(greet)]);
	$.append($$anchor, p);
	$.bind_prop($$props, "greet", greet);
	return $.pop($$exports);
}
