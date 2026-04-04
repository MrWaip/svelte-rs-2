import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let promise = $.proxy(Promise.resolve(42));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, ($$anchor, value) => {
		const doubled = $.derived(() => $.get(value) * 2);
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(doubled)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
