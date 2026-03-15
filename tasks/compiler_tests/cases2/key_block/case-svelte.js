import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let count = $.state(0);
	$.set(count, 1);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.key(node, () => $.get(count), ($$anchor) => {
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(count)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
