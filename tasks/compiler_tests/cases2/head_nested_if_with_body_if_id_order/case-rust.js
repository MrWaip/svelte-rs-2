import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<meta name="a" content="b"/>`);
var root_3 = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	let cond = $.prop($$props, "cond", 3, true), show = $.prop($$props, "show", 3, true);
	var fragment_1 = $.comment();
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var meta = root_2();
				$.append($$anchor, meta);
			};
			$.if(node, ($$render) => {
				if (show()) $$render(consequent);
			});
		}
		$.append($$anchor, fragment);
	});
	var node_1 = $.first_child(fragment_1);
	{
		var consequent_1 = ($$anchor) => {
			var button = root_3();
			$.append($$anchor, button);
		};
		$.if(node_1, ($$render) => {
			if (cond()) $$render(consequent_1);
		});
	}
	$.append($$anchor, fragment_1);
}
