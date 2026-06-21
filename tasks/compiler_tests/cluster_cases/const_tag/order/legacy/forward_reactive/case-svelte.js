import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let n = $.mutable_source(1);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const bar = $.derived_safe_equal(() => $.get(n));
			const foo = $.derived_safe_equal(() => $.get(bar));
			var button = root_1();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, $.get(foo)));
			$.event("click", button, () => $.set(n, $.get(n) + 1));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
