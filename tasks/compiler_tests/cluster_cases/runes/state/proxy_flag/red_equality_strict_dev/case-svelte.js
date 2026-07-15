import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let eq = $.state($$props.a === $$props.b);
	function toggle() {
		$.set(eq, !$.get(eq));
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(eq)));
	$.delegated("click", button, toggle);
	$.append($$anchor, button);
}
$.delegate(["click"]);
