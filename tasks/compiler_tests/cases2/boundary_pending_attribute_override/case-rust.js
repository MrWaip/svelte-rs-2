import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>loading...</p>`);
var root_2 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	function pending($$anchor) {
		console.log("attribute", $$anchor);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		$.boundary(node, {
			pending,
			pending
		}, ($$anchor) => {
			var p_1 = root_2();
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
}
