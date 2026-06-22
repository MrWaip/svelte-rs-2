import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let n = "world";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const bar = $.derived(() => n);
			const foo = $.derived(() => $.get(bar));
			var p = root_1();
			p.textContent = $.get(foo);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (n) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
