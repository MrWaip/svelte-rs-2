import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let count = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			App(node_1, { count: count - 1 });
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (count > 0) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
