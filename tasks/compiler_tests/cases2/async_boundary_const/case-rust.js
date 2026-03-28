import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let x = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor, error = $.noop) => {
			const doubled = $.derived(() => x * 2);
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			const doubled = $.derived(() => x * 2);
			var p_1 = root_2();
			var text_1 = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text_1, $.get(doubled)));
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
}
