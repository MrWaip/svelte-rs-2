App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <!> <div><p></p></div>`, 1), App[$.FILENAME], [[
	15,
	0,
	[[17, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	let x = 42;
	let items = [
		1,
		2,
		3
	];
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.template_effect(() => {
				console.log({ x: $.snapshot(x) });
				debugger;
			});
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 7, 0);
	}
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.each(node_1, 17, () => items, $.index, ($$anchor, item) => {
		$.template_effect(() => {
			console.log({ item: $.snapshot($.get(item)) });
			debugger;
		});
	}), "each", App, 11, 0);
	var div = $.sibling(node_1, 2);
	$.template_effect(() => {
		console.log({ x: $.snapshot(x) });
		debugger;
	});
	var p = $.child(div);
	p.textContent = "Value: 42";
	$.reset(div);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
