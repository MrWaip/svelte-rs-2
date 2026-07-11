import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const DEFAULTS = { a: 1 };
	let config = $.prop($$props, "config", 27, () => $.proxy(DEFAULTS));
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, config().a));
	$.append($$anchor, button);
	$.pop();
}
