import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let obj = $.proxy({ x: null });
	let src = $.proxy({});
	let depth = 0;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			App(node_1, {
				onChange: (v) => obj.x = src,
				depth: depth - 1
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (depth) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
