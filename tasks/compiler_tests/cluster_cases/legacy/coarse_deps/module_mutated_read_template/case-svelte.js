import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
let count = 0;
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	function bump() {
		count = count + 1;
	}
	var $$exports = { bump };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count));
	$.append($$anchor, p);
	$.bind_prop($$props, "bump", bump);
	return $.pop($$exports);
}
