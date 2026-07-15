import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>a</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor) => {
			$.next();
			var text = $.text("failed text");
			$.append($$anchor, text);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
}
