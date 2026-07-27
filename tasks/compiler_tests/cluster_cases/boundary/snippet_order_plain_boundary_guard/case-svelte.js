import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const other = ($$anchor) => {
			$.next();
			var text = $.text("x");
			$.append($$anchor, text);
		};
		$.boundary(node, {}, ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = ($$anchor) => {
					$.next();
					var text_1 = $.text("y");
					$.append($$anchor, text_1);
				};
				$.boundary(node_1, { failed }, ($$anchor) => {});
			}
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
}
