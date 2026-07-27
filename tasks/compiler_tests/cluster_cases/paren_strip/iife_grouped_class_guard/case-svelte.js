import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let a = $.state(0);
	const c = $.derived(() => class {}($.get(a)));
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, typeof $.get(c)));
	$.delegated("click", button, () => $.update(a));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
