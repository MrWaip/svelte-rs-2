import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor, error = $.noop) => {
			const x = $.derived(() => items.length);
			var p = root_1();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}: ${error().message ?? ""}`));
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			const x = $.derived(() => items.length);
			var p_1 = root_2();
			var text_1 = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text_1, $.get(x)));
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
}
