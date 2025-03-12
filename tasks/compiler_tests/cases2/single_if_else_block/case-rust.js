import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("if text");
			$.append($$anchor, text);
		};
		var alternate = ($$anchor, $$elseif) => {
			{
				var consequent_1 = ($$anchor) => {
					var text_1 = $.text("else if text");
					$.append($$anchor, text_1);
				};
				var alternate_1 = ($$anchor) => {
					var text_2 = $.text("else text");
					$.append($$anchor, text_2);
				};
				$.if($$anchor, ($$render) => {
					if (false) $$render(consequent_1);
else $$render(alternate_1, false);
				}, $$elseif);
			}
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
else $$render(alternate, false);
		});
	}
	$.append($$anchor, fragment);
}
