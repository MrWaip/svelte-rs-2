import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let sum = $.state($$props.a + $$props.b);
	function inc() {
		$.set(sum, $.get(sum) + 1);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(sum)));
	$.delegated("click", button, inc);
	$.append($$anchor, button);
}
$.delegate(["click"]);
