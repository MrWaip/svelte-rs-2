import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>loading...</p>`);
var root_2 = $.from_html(`<p> </p>`);
var root_3 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	function handleError(error) {
		console.error(error);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		const failed = ($$anchor, error = $.noop) => {
			var p_1 = root_2();
			var text = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text, error().message));
			$.append($$anchor, p_1);
		};
		$.boundary(node, {
			onerror: handleError,
			pending,
			failed
		}, ($$anchor) => {
			var p_2 = root_3();
			$.append($$anchor, p_2);
		});
	}
	$.append($$anchor, fragment);
}
