import * as $ from "svelte/internal/client";
var root_2 = $.template(`<div><div></div> <button></button></div>`);
var root_4 = $.template(`<div><p>Lorem</p></div>`);
var root_5 = $.template(`<h2>Old UI</h2>`);
var root_1 = $.template(`<div><!></div>`);
var root_6 = $.template(`<div></div>`);
var root = $.template(`<h1><span></span> <button>+</button> some long text</h1> <noscript></noscript> <!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var h1 = $.first_child(fragment);
	var span = $.child(h1);
	span.textContent = `Entities ${logged_in ?? ""}`;
	$.next(3);
	$.reset(h1);
	var node = $.sibling(h1, 4);
	{
		var consequent_2 = ($$anchor) => {
			var div = root_1();
			var node_1 = $.child(div);
			{
				var consequent = ($$anchor) => {
					var div_1 = root_2();
					var div_2 = $.child(div_1);
					div_2.textContent = user_name;
					var button = $.sibling(div_2, 2);
					button.textContent = counter;
					$.reset(div_1);
					$.append($$anchor, div_1);
				};
				var alternate = ($$anchor, $$elseif) => {
					{
						var consequent_1 = ($$anchor) => {
							var div_3 = root_4();
							$.append($$anchor, div_3);
						};
						var alternate_1 = ($$anchor) => {
							var h2 = root_5();
							$.append($$anchor, h2);
						};
						$.if($$anchor, ($$render) => {
							if (featureB) $$render(consequent_1);
else $$render(alternate_1, false);
						}, $$elseif);
					}
				};
				$.if(node_1, ($$render) => {
					if (featureA) $$render(consequent);
else $$render(alternate, false);
				});
			}
			$.reset(div);
			$.append($$anchor, div);
		};
		var alternate_2 = ($$anchor) => {
			var div_4 = root_6();
			div_4.textContent = `Spinner ${percent ?? ""}`;
			$.append($$anchor, div_4);
		};
		$.if(node, ($$render) => {
			if (!loading) $$render(consequent_2);
else $$render(alternate_2, false);
		});
	}
	$.append($$anchor, fragment);
}
