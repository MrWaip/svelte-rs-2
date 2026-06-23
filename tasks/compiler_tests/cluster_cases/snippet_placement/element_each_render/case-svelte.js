import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let numbers = $.proxy([
		1,
		2,
		3
	]);
	var div = root();
	{
		const x = ($$anchor, n = $.noop) => {
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, n()));
			$.append($$anchor, p);
		};
		$.each(div, 21, () => numbers, $.index, ($$anchor, n) => {
			x($$anchor, () => $.get(n));
		});
		$.reset(div);
	}
	$.append($$anchor, div);
}
