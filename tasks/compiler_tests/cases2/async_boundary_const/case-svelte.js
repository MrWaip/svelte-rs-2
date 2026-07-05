import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let x = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor, error = $.noop) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			const doubled = $.derived(() => x * 2);
			var p_1 = root_1();
			p_1.textContent = $.get(doubled);
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
}
