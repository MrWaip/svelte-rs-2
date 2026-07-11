import * as $ from "svelte/internal/client";
var root = $.from_html(`<span>loading</span>`);
var root_1 = $.from_html(`<p>a</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = ($$anchor) => {
			var span = root();
			$.append($$anchor, span);
		};
		$.boundary(node, { pending }, ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
}
