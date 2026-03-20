import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let show = true;
	let tag = "div";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.element(node_1, () => tag, false, ($$element, $$anchor) => {
				var text = $.text("content");
				$.append($$anchor, text);
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
