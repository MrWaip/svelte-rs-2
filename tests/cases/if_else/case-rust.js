import * as $ from "svelte/internal/client";
var root_3 = $.template(`<br> Text 2`, 1);
var root_4 = $.template(`<br> text 3`, 1);
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
					var fragment_2 = root_3();
					$.next();
					$.append($$anchor, fragment_2);
				};
				var alternate = ($$anchor) => {
					var fragment_3 = root_4();
					$.next();
					$.append($$anchor, fragment_3);
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
