import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button> <p> </p>`, 1);
export default function App($$anchor) {
	let r = $.state(/ab/);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(r)));
	$.delegated("click", button, () => $.set(r, /cd/));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
