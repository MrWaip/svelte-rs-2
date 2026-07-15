import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>a</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = ($$anchor) => {
			$.next();
			var text = $.text("loading...");
			$.append($$anchor, text);
		};
		$.boundary(node, { pending }, ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
}
