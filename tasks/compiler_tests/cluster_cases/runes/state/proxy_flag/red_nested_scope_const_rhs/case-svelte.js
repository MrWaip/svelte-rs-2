import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let value = $.state(0);
	const handler = () => {
		const local = 5;
		$.set(value, local);
	};
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, handler);
	$.append($$anchor, button);
}
$.delegate(["click"]);
