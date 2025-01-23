import * as $ from "svelte/internal/client";
var root_1 = $.template(`<br> Text 1`, 1);
var root_3 = $.template(`<br> Text 2`, 1);
var root_4 = $.template(`<br> text 3`, 1);
var root = $.template(`<input><!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment));
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root_1();
			$.next();
			$.append($$anchor, fragment_1);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_2 = $.comment();
			var node_1 = $.first_child(fragment_2);
			{
				var consequent_1 = ($$anchor) => {
					var fragment_3 = root_3();
					$.next();
					$.append($$anchor, fragment_3);
				};
				var alternate = ($$anchor) => {
					var fragment_4 = root_4();
					$.next();
					$.append($$anchor, fragment_4);
				};
				$.if(node_1, ($$render) => {
					if (1 + 2) $$render(consequent_1);
else $$render(alternate, false);
				}, true);
			}
			$.append($$anchor, fragment_2);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
else $$render(alternate_1, false);
		});
	}
	$.append($$anchor, fragment);
}
