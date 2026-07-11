import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let flag = $.prop($$props, "flag", 3, false);
	let value = $.state(0);
	function apply() {
		$.set(value, flag());
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, apply);
	$.append($$anchor, button);
}
$.delegate(["click"]);
