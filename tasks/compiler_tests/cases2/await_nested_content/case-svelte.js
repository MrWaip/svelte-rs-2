import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div class="result"><h1>Result</h1> <p> </p></div>`);
var root_2 = $.from_html(`<div class="error"><h1>Error</h1> <p> </p></div>`);
var root_3 = $.from_html(`<div class="loading"><span>Please wait...</span></div>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, ($$anchor) => {
		var div_2 = root_3();
		$.append($$anchor, div_2);
	}, ($$anchor, value) => {
		var div = root_1();
		var p = $.sibling($.child(div), 2);
		var text = $.child(p, true);
		$.reset(p);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, div);
	}, ($$anchor, error) => {
		var div_1 = root_2();
		var p_1 = $.sibling($.child(div_1), 2);
		var text_1 = $.child(p_1, true);
		$.reset(p_1);
		$.reset(div_1);
		$.template_effect(() => $.set_text(text_1, $.get(error).message));
		$.append($$anchor, div_1);
	});
	$.append($$anchor, fragment);
}
