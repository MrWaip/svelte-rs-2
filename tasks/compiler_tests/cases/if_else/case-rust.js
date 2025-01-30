import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("Text 1");
			$.append($$anchor, text);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				var consequent_1 = ($$anchor) => {
					var text_1 = $.text("Text 2");
					$.append($$anchor, text_1);
				};
				var alternate = ($$anchor) => {
					var text_2 = $.text("text 3");
					$.append($$anchor, text_2);
				};
				$.if(node_1, ($$render) => {
					if (1 + 2) $$render(consequent_1);
else $$render(alternate, false);
				}, true);
			}
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
else $$render(alternate_1, false);
		});
	}
	$.append($$anchor, fragment);
}
