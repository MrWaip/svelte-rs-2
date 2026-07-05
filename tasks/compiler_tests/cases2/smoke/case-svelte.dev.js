App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><div></div> <button></button></div>`), App[$.FILENAME], [[
	13,
	12,
	[[14, 16], [18, 16]]
]]);
var root_1 = $.add_locations($.from_html(`<div><p>Lorem</p></div>`), App[$.FILENAME], [[
	21,
	12,
	[[22, 16]]
]]);
var root_2 = $.add_locations($.from_html(`<h2>Old UI</h2>`), App[$.FILENAME], [[25, 12]]);
var root_3 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[11, 4]]);
var root_4 = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[31, 4]]);
var root_5 = $.add_locations($.from_html(`<h1><span></span> <button>+</button> some long text</h1> <noscript></noscript> <!>`, 1), App[$.FILENAME], [[
	1,
	0,
	[[2, 4], [3, 4]]
], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_5();
	var h1 = $.first_child(fragment);
	var span = $.child(h1);
	span.textContent = `Entities ${logged_in ?? ""}`;
	$.next(3);
	$.reset(h1);
	var node = $.sibling(h1, 4);
	{
		var consequent_2 = ($$anchor) => {
			var div = root_3();
			var node_1 = $.child(div);
			{
				var consequent = ($$anchor) => {
					var div_1 = root();
					var div_2 = $.child(div_1);
					div_2.textContent = user_name;
					var button = $.sibling(div_2, 2);
					button.textContent = counter;
					$.reset(div_1);
					$.append($$anchor, div_1);
				};
				var consequent_1 = ($$anchor) => {
					var div_3 = root_1();
					$.append($$anchor, div_3);
				};
				var alternate = ($$anchor) => {
					var h2 = root_2();
					$.append($$anchor, h2);
				};
				$.add_svelte_meta(() => $.if(node_1, ($$render) => {
					if (featureA) $$render(consequent);
					else if (featureB) $$render(consequent_1, 1);
					else $$render(alternate, -1);
				}), "if", App, 12, 8);
			}
			$.reset(div);
			$.append($$anchor, div);
		};
		var alternate_1 = ($$anchor) => {
			var div_4 = root_4();
			div_4.textContent = `Spinner ${percent ?? ""}`;
			$.append($$anchor, div_4);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (!loading) $$render(consequent_2);
			else $$render(alternate_1, -1);
		}), "if", App, 10, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
