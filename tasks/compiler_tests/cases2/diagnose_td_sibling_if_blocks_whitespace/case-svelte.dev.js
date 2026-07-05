App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(` <br/>`, 1), App[$.FILENAME], [[10, 24]]);
var root_1 = $.add_locations($.from_html(` <br/>`, 1), App[$.FILENAME], [[13, 24]]);
var root_2 = $.add_locations($.from_html(`<table><tbody><tr><td><!> <!></td></tr></tbody></table>`), App[$.FILENAME], [[
	5,
	0,
	[[
		6,
		4,
		[[
			7,
			8,
			[[8, 12]]
		]]
	]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var table = root_2();
	var tbody = $.child(table);
	var tr = $.child(tbody);
	var td = $.child(tr);
	var node = $.child(td);
	{
		var consequent = ($$anchor) => {
			var fragment = root();
			var text = $.first_child(fragment);
			$.next();
			$.template_effect(() => $.set_text(text, `${$$props.a ?? ""} `));
			$.append($$anchor, fragment);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.a) $$render(consequent);
		}), "if", App, 9, 16);
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent_1 = ($$anchor) => {
			var fragment_1 = root_1();
			var text_1 = $.first_child(fragment_1);
			$.next();
			$.template_effect(() => $.set_text(text_1, `${$$props.b ?? ""} `));
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if ($$props.b) $$render(consequent_1);
		}), "if", App, 12, 16);
	}
	$.reset(td);
	$.reset(tr);
	$.reset(tbody);
	$.reset(table);
	$.append($$anchor, table);
	return $.pop($$exports);
}
