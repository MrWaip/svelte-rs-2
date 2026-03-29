import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span></span>`);
var root_3 = $.from_html(`<h1>Big</h1>`);
var root_4 = $.from_html(`<h2>Small</h2>`);
var root_2 = $.from_html(`<div><input/></div> <!>`, 1);
var root_5 = $.from_html(`<span></span>`);
var root_7 = $.from_html(`<h1>Big</h1>`);
var root_8 = $.from_html(`<h2>Small</h2>`);
var root_6 = $.from_html(`<div><input/></div> <!>`, 1);
var root = $.from_html(`<div><!></div> <div><!></div>`, 1);
export default function App($$anchor) {
	let count = 0;
	let visible = true;
	var fragment = root();
	var div = $.first_child(fragment);
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var span = root_1();
			span.textContent = "0";
			$.append($$anchor, span);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = root_2();
			var div_1 = $.first_child(fragment_1);
			var input = $.child(div_1);
			$.remove_input_defaults(input);
			$.set_value(input, count);
			$.reset(div_1);
			var node_1 = $.sibling(div_1, 2);
			{
				var consequent_1 = ($$anchor) => {
					var h1 = root_3();
					$.append($$anchor, h1);
				};
				var alternate = ($$anchor) => {
					var h2 = root_4();
					$.append($$anchor, h2);
				};
				$.if(node_1, ($$render) => {
					if (count > 10) $$render(consequent_1);
					else $$render(alternate, -1);
				});
			}
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
			else $$render(alternate_1, -1);
		});
	}
	$.reset(div);
	var div_2 = $.sibling(div, 2);
	var node_2 = $.child(div_2);
	{
		var consequent_2 = ($$anchor) => {
			var span_1 = root_5();
			span_1.textContent = "0";
			$.append($$anchor, span_1);
		};
		var alternate_3 = ($$anchor) => {
			var fragment_2 = root_6();
			var div_3 = $.first_child(fragment_2);
			var input_1 = $.child(div_3);
			$.remove_input_defaults(input_1);
			$.set_value(input_1, count);
			$.reset(div_3);
			var node_3 = $.sibling(div_3, 2);
			{
				var consequent_3 = ($$anchor) => {
					var h1_1 = root_7();
					$.append($$anchor, h1_1);
				};
				var alternate_2 = ($$anchor) => {
					var h2_1 = root_8();
					$.append($$anchor, h2_1);
				};
				$.if(node_3, ($$render) => {
					if (count > 10) $$render(consequent_3);
					else $$render(alternate_2, -1);
				});
			}
			$.append($$anchor, fragment_2);
		};
		$.if(node_2, ($$render) => {
			if (visible) $$render(consequent_2);
			else $$render(alternate_3, -1);
		});
	}
	$.reset(div_2);
	$.append($$anchor, fragment);
}
