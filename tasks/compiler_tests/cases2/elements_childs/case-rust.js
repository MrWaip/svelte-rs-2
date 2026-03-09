import * as $ from "svelte/internal/client";
var root_1 = $.template(`<div></div>`);
var root_3 = $.template(`<div></div>`);
var root_4 = $.template(`<div></div>`);
var root = $.template(`<div>text only</div> <div></div> <div></div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div><!></div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var div = $.sibling($.first_child(fragment), 2);
	div.textContent = interpolation;
	var div_1 = $.sibling(div, 2);
	div_1.textContent = `concatenated + ${interpolation ?? ""} + concatenated`;
	var div_2 = $.sibling(div_1, 4);
	var node = $.child(div_2);
	{
		var consequent = ($$anchor) => {
			var div_3 = root_1();
			$.append($$anchor, div_3);
		};
		var alternate = ($$anchor, $$elseif) => {
			{
				var consequent_1 = ($$anchor) => {
					var div_4 = root_3();
					$.append($$anchor, div_4);
				};
				var alternate_1 = ($$anchor) => {
					var div_5 = root_4();
					$.append($$anchor, div_5);
				};
				$.if($$anchor, ($$render) => {
					if (2 === 2) $$render(consequent_1);
else $$render(alternate_1, false);
				}, $$elseif);
			}
		};
		$.if(node, ($$render) => {
			if (1 !== 1) $$render(consequent);
else $$render(alternate, false);
		});
	}
	$.reset(div_2);
	$.next(2);
	$.append($$anchor, fragment);
}
