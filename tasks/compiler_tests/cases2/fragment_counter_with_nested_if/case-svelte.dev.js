App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span></span>`), App[$.FILENAME], [[8, 8]]);
var root_1 = $.add_locations($.from_html(`<h1>Big</h1>`), App[$.FILENAME], [[15, 12]]);
var root_2 = $.add_locations($.from_html(`<h2>Small</h2>`), App[$.FILENAME], [[17, 12]]);
var root_3 = $.add_locations($.from_html(`<div><input/></div> <!>`, 1), App[$.FILENAME], [[
	10,
	8,
	[[11, 12]]
]]);
var root_4 = $.add_locations($.from_html(`<span></span>`), App[$.FILENAME], [[24, 8]]);
var root_5 = $.add_locations($.from_html(`<h1>Big</h1>`), App[$.FILENAME], [[31, 12]]);
var root_6 = $.add_locations($.from_html(`<h2>Small</h2>`), App[$.FILENAME], [[33, 12]]);
var root_7 = $.add_locations($.from_html(`<div><input/></div> <!>`, 1), App[$.FILENAME], [[
	26,
	8,
	[[27, 12]]
]]);
var root_8 = $.add_locations($.from_html(`<div><!></div> <div><!></div>`, 1), App[$.FILENAME], [[6, 0], [22, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	let visible = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_8();
	var div = $.first_child(fragment);
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var span = root();
			span.textContent = "0";
			$.append($$anchor, span);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = root_3();
			var div_1 = $.first_child(fragment_1);
			var input = $.child(div_1);
			$.remove_input_defaults(input);
			$.set_value(input, count);
			$.reset(div_1);
			var node_1 = $.sibling(div_1, 2);
			{
				var consequent_1 = ($$anchor) => {
					var h1 = root_1();
					$.append($$anchor, h1);
				};
				var alternate = ($$anchor) => {
					var h2 = root_2();
					$.append($$anchor, h2);
				};
				$.add_svelte_meta(() => $.if(node_1, ($$render) => {
					if (count > 10) $$render(consequent_1);
					else $$render(alternate, -1);
				}), "if", App, 14, 8);
			}
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (visible) $$render(consequent);
			else $$render(alternate_1, -1);
		}), "if", App, 7, 4);
	}
	$.reset(div);
	var div_2 = $.sibling(div, 2);
	var node_2 = $.child(div_2);
	{
		var consequent_2 = ($$anchor) => {
			var span_1 = root_4();
			span_1.textContent = "0";
			$.append($$anchor, span_1);
		};
		var alternate_3 = ($$anchor) => {
			var fragment_2 = root_7();
			var div_3 = $.first_child(fragment_2);
			var input_1 = $.child(div_3);
			$.remove_input_defaults(input_1);
			$.set_value(input_1, count);
			$.reset(div_3);
			var node_3 = $.sibling(div_3, 2);
			{
				var consequent_3 = ($$anchor) => {
					var h1_1 = root_5();
					$.append($$anchor, h1_1);
				};
				var alternate_2 = ($$anchor) => {
					var h2_1 = root_6();
					$.append($$anchor, h2_1);
				};
				$.add_svelte_meta(() => $.if(node_3, ($$render) => {
					if (count > 10) $$render(consequent_3);
					else $$render(alternate_2, -1);
				}), "if", App, 30, 8);
			}
			$.append($$anchor, fragment_2);
		};
		$.add_svelte_meta(() => $.if(node_2, ($$render) => {
			if (visible) $$render(consequent_2);
			else $$render(alternate_3, -1);
		}), "if", App, 23, 4);
	}
	$.reset(div_2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
