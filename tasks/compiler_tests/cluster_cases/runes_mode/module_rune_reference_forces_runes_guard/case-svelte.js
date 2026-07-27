import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>inc</button>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	let doubled = $.derived(() => $.get(count) * 2);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
