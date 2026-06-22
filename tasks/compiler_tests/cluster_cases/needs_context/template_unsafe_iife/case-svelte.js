import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let n = $.state(0);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, $0), [() => (() => $.get(n))()]);
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
