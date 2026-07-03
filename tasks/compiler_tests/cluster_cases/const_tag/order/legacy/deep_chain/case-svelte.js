import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const c = $.derived_safe_equal(() => "x");
			const b = $.derived_safe_equal(() => $.get(c));
			const a = $.derived_safe_equal(() => $.get(b));
			var p = root();
			p.textContent = $.get(a);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
