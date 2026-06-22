import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <div><span> </span></div>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	var span = $.child(div);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
