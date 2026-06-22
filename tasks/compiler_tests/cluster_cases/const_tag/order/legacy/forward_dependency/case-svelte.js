import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<h1></h1>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const bar = $.derived_safe_equal(() => "world");
			const foo = $.derived_safe_equal(() => $.get(bar));
			const yoo = $.derived_safe_equal(() => $.get(foo));
			var h1 = root_1();
			h1.textContent = `Hello ${$.get(bar) ?? ""}${$.get(yoo) ?? ""}!`;
			$.append($$anchor, h1);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
