import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const a = $.derived_safe_equal(() => "x");
			const b = $.derived_safe_equal(() => "y");
			var p = root_1();
			p.textContent = `${$.get(a) ?? ""}${$.get(b) ?? ""}`;
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
