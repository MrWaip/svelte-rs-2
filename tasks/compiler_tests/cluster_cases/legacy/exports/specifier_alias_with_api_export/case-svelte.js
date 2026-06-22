import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let className = $.prop($$props, "class", 8, "btn");
	function getClass() {
		return className();
	}
	var $$exports = { getClass };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, className()));
	$.append($$anchor, p);
	$.bind_prop($$props, "getClass", getClass);
	return $.pop($$exports);
}
