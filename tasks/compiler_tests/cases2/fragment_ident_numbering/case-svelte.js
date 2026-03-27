import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span></span>`);
var root_2 = $.from_html(`<div><input/></div>`);
var root_3 = $.from_html(`<span></span>`);
var root_4 = $.from_html(`<div><input/></div>`);
var root = $.from_html(`<div><!></div> <div><!></div>`, 1);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	let state = "test";
	var fragment = root();
	var div = $.first_child(fragment);
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var span = root_1();
			span.textContent = "test";
			$.append($$anchor, span);
		};
		var alternate = ($$anchor) => {
			var div_1 = root_2();
			var input = $.child(div_1);
			$.remove_input_defaults(input);
			$.set_value(input, state);
			$.reset(div_1);
			$.append($$anchor, div_1);
		};
		$.if(node, ($$render) => {
			if (state) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.reset(div);
	var div_2 = $.sibling(div, 2);
	var node_1 = $.child(div_2);
	{
		var consequent_1 = ($$anchor) => {
			var span_1 = root_3();
			span_1.textContent = "test";
			$.append($$anchor, span_1);
		};
		var alternate_1 = ($$anchor) => {
			var div_3 = root_4();
			var input_1 = $.child(div_3);
			$.remove_input_defaults(input_1);
			$.set_value(input_1, state);
			$.reset(div_3);
			$.append($$anchor, div_3);
		};
		$.if(node_1, ($$render) => {
			if (state) $$render(consequent_1);
			else $$render(alternate_1, -1);
		});
	}
	$.reset(div_2);
	$.append($$anchor, fragment);
}
