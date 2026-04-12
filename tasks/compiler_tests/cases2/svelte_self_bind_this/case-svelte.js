import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let ref;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.bind_this(App(node_1, { answer: 42 }), ($$value) => ref = $$value, () => ref);
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
