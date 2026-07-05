import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<p>guarded</p>`);
export default function App($$anchor) {
	let show = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = ($$anchor, error = $.noop) => {
					var p = root();
					var text = $.child(p, true);
					$.reset(p);
					$.template_effect(() => $.set_text(text, error().message));
					$.append($$anchor, p);
				};
				$.boundary(node_1, { failed }, ($$anchor) => {
					var p_1 = root_1();
					$.append($$anchor, p_1);
				});
			}
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
