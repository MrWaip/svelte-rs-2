import * as $ from "svelte/internal/client";
var root_1 = $.from_html(` <br/>`, 1);
var root_2 = $.from_html(` <br/>`, 1);
var root = $.from_html(`<table><tbody><tr><td><!> <!></td></tr></tbody></table>`);
export default function App($$anchor, $$props) {
	var table = root();
	var tbody = $.child(table);
	var tr = $.child(tbody);
	var td = $.child(tr);
	var node = $.child(td);
	{
		var consequent = ($$anchor) => {
			var fragment = root_1();
			var text = $.first_child(fragment);
			$.next();
			$.template_effect(() => $.set_text(text, `${$$props.a ?? ""} `));
			$.append($$anchor, fragment);
		};
		$.if(node, ($$render) => {
			if ($$props.a) $$render(consequent);
		});
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent_1 = ($$anchor) => {
			var fragment_1 = root_2();
			var text_1 = $.first_child(fragment_1);
			$.next();
			$.template_effect(() => $.set_text(text_1, `${$$props.b ?? ""} `));
			$.append($$anchor, fragment_1);
		};
		$.if(node_1, ($$render) => {
			if ($$props.b) $$render(consequent_1);
		});
	}
	$.reset(td);
	$.reset(tr);
	$.reset(tbody);
	$.reset(table);
	$.append($$anchor, table);
}
