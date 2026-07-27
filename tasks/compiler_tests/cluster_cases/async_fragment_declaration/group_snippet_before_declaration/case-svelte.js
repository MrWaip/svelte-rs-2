import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<b> </b>`);
var root_1 = $.from_html(`<div><span> </span> <!></div>`);
export default function App($$anchor) {
	const id = "name";
	var div = root_1();
	{
		const greet = ($$anchor, x = $.noop) => {
			var b = root();
			var text = $.child(b, true);
			$.reset(b);
			$.template_effect(() => $.set_text(text, x()));
			$.append($$anchor, b);
		};
		let greeting2;
		var promises = $.run([async () => greeting2 = await $.async_derived(() => `Hi ${id}`)]);
		var span = $.child(div);
		var text_1 = $.child(span, true);
		$.reset(span);
		var node = $.sibling(span, 2);
		greet(node, () => 1);
		$.reset(div);
		$.template_effect(() => $.set_text(text_1, $.get(greeting2)), void 0, void 0, [promises[0]]);
	}
	$.append($$anchor, div);
}
