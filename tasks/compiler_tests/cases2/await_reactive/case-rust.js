import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let url = "/api";
	let promise = $.derived(() => fetch(url));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => $.get(promise), null, ($$anchor, value) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
