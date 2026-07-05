import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<p>Loading...</p>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, ($$anchor) => {
		var p_2 = root_1();
		$.append($$anchor, p_2);
	}, ($$anchor, value) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, p);
	}, ($$anchor, error) => {
		var p_1 = root();
		var text_1 = $.child(p_1, true);
		$.reset(p_1);
		$.template_effect(() => $.set_text(text_1, $.get(error).message));
		$.append($$anchor, p_1);
	});
	$.append($$anchor, fragment);
}
