import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
var root_2 = $.from_html(`<p>Done</p>`);
var root = $.from_html(`<p> </p> <!>`, 1);
export default function App($$anchor) {
	let count = 0;
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var node = $.sibling(p, 2);
	{
		var consequent = ($$anchor) => {
			var p_1 = root_1();
			p_1.textContent = "Loading 0";
			$.append($$anchor, p_1);
		};
		var alternate = ($$anchor) => {
			var p_2 = root_2();
			$.append($$anchor, p_2);
		};
		$.if(node, ($$render) => {
			if ($.eager($.pending)) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.template_effect(() => $.set_text(text, $.eager($.pending)));
	$.append($$anchor, fragment);
}
