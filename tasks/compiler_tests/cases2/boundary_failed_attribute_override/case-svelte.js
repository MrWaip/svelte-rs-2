import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	function failed($$anchor, error) {
		console.log("attribute", error);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor, error = $.noop) => {
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, p);
		};
		$.boundary(node, {
			failed,
			failed
		}, ($$anchor) => {
			var p_1 = root_2();
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
}
