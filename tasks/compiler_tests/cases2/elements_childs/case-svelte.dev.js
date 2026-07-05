App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[20, 4]]);
var root_1 = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[22, 4]]);
var root_2 = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[24, 4]]);
var root_3 = $.add_locations($.from_html(`<div>text only</div> <div></div> <div></div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div><!></div> <div></div>`, 1), App[$.FILENAME], [
	[1, 0],
	[3, 0],
	[7, 0],
	[
		11,
		0,
		[
			[12, 4],
			[13, 4],
			[14, 4]
		]
	],
	[17, 0],
	[29, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_3();
	var div = $.sibling($.first_child(fragment), 2);
	div.textContent = interpolation;
	var div_1 = $.sibling(div, 2);
	div_1.textContent = `concatenated + ${interpolation ?? ""} + concatenated`;
	var div_2 = $.sibling(div_1, 4);
	var node = $.child(div_2);
	{
		var consequent = ($$anchor) => {
			var div_3 = root();
			$.append($$anchor, div_3);
		};
		var consequent_1 = ($$anchor) => {
			var div_4 = root_1();
			$.append($$anchor, div_4);
		};
		var alternate = ($$anchor) => {
			var div_5 = root_2();
			$.append($$anchor, div_5);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.strict_equals(1, 1, false)) $$render(consequent);
			else if ($.strict_equals(2, 2)) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		}), "if", App, 19, 0);
	}
	$.reset(div_2);
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
