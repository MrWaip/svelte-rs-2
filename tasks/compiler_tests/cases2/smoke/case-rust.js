import * as $ from "svelte/internal/client";
var root = $.template(`<h1><span></span><button>+</button> </h1><noscript>any content</noscript><!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	{
		var consequent = ($$anchor) => {
			var root_1 = $.template(`<div><!></div>`);
			var div = root_1();
			var node_1 = $.child(div);
			{
				var consequent_1 = ($$anchor) => {
					var root_2 = $.template(`<div><div></div><button></button></div>`);
					var div_1 = root_2();
					$.append($$anchor, div_1);
				};
				var alternate = ($$anchor, $$elseif) => {
					{
						var consequent_2 = ($$anchor) => {
							var root_3 = $.template(`<div><p>Lorem</p></div>`);
							var div_2 = root_3();
							$.append($$anchor, div_2);
						};
						var alternate_1 = ($$anchor) => {
							var root_4 = $.template(`<h2>Old UI</h2>`);
							var h2 = root_4();
							$.append($$anchor, h2);
						};
						$.if($$anchor, ($$render) => {
							if (featureB) $$render(consequent_2);
else $$render(alternate_1, false);
						}, $$elseif);
					}
				};
				$.if(node_1, ($$render) => {
					if (featureA) $$render(consequent_1);
else $$render(alternate, false);
				});
			}
			$.append($$anchor, div);
		};
		var alternate_2 = ($$anchor) => {
			var root_5 = $.template(`<div></div>`);
			var div_3 = root_5();
			div_3.textContent = `Spinner ${percent ?? ""}`;
			$.append($$anchor, div_3);
		};
		$.if(node, ($$render) => {
			if (!loading) $$render(consequent);
else $$render(alternate_2, false);
		});
	}
	$.append($$anchor, fragment);
}
