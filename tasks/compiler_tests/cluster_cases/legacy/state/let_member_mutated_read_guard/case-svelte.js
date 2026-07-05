import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let obj = $.mutable_source({ k: 1 });
	const bump = () => {
		$.mutate(obj, $.get(obj).k = 2);
	};
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(obj), $.untrack(() => $.get(obj).k))));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
