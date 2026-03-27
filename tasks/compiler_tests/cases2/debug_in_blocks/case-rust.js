import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <!> <div><p></p></div>`, 1);
export default function App($$anchor) {
	let show = true;
	let x = 42;
	let items = [
		1,
		2,
		3
	];
	var fragment = root();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.template_effect(() => {
				console.log({ x: $.snapshot(x) });
				debugger;
			});
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	var node_1 = $.sibling(node, 2);
	$.each(node_1, 17, () => items, $.index, ($$anchor, item) => {
		$.template_effect(() => {
			console.log({ item: $.snapshot($.get(item)) });
			debugger;
		});
	});
	var div = $.sibling(node_1, 2);
	$.template_effect(() => {
		console.log({ x: $.snapshot(x) });
		debugger;
	});
	var p = $.child(div);
	p.textContent = "Value: 42";
	$.reset(div);
	$.append($$anchor, fragment);
}
