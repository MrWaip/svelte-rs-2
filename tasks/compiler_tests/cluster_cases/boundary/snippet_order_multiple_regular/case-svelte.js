import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const a = ($$anchor) => {
			$.next();
			var text = $.text("1");
			$.append($$anchor, text);
		};
		const b = ($$anchor) => {
			$.next();
			var text_1 = $.text("2");
			$.append($$anchor, text_1);
		};
		const failed = ($$anchor) => {
			$.next();
			var text_2 = $.text("z");
			$.append($$anchor, text_2);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = ($$anchor) => {
					$.next();
					var text_3 = $.text("y");
					$.append($$anchor, text_3);
				};
				$.boundary(node_1, { failed }, ($$anchor) => {});
			}
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
}
