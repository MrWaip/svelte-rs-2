import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root_3 = $.from_html(`<p> </p>`);
var root_4 = $.from_html(`<p>inner</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor, error = $.noop) => {
			var p = root_1();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `outer: ${error().message ?? ""}`));
			$.append($$anchor, p);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				const failed = ($$anchor, error = $.noop) => {
					var p_1 = root_3();
					var text_1 = $.child(p_1, true);
					$.reset(p_1);
					$.template_effect(() => $.set_text(text_1, error().message));
					$.append($$anchor, p_1);
				};
				$.boundary(node_1, { failed }, ($$anchor) => {
					var p_2 = root_4();
					$.append($$anchor, p_2);
				});
			}
			$.append($$anchor, fragment_1);
		});
	}
	$.append($$anchor, fragment);
}
