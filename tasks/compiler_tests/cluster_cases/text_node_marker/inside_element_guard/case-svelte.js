import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> <br/></p> <button>inc</button>`, 1);
export default function App($$anchor) {
	let a = $.state(0);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.next();
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.delegated("click", button, () => $.update(a));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
