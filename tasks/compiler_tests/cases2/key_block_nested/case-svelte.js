import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<div>before <!> after</div>`);
export default function App($$anchor) {
	let count = $.state(0);
	$.set(count, 1);
	var div = root_1();
	var node = $.sibling($.child(div));
	$.key(node, () => $.get(count) % 2, ($$anchor) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(count)));
		$.append($$anchor, span);
	});
	$.next();
	$.reset(div);
	$.append($$anchor, div);
}
