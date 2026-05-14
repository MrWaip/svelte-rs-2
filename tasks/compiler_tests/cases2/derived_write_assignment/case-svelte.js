import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let invalid = $.derived(() => Boolean($$props.flag));
	function reset() {
		$.set(invalid, false);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(invalid)));
	$.delegated("click", button, reset);
	$.append($$anchor, button);
}
$.delegate(["click"]);
