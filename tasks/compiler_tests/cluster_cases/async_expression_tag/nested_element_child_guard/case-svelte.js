import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p><span> </span></p>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var span = $.child(p);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), void 0, [() => delay($.get(x))]);
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
