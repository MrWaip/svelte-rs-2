import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let p = $.state(null);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(p)));
	$.delegated("click", button, () => $.set(p, new Promise(() => {}), true));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
