import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = ($$anchor) => {
			const foo = $.derived(() => "bar");
			$.next();
			var text = $.text();
			text.nodeValue = $.get(foo);
			$.append($$anchor, text);
		};
		$.boundary(node, { failed }, ($$anchor) => {
			const foo = $.derived(() => "bar");
		});
	}
	$.append($$anchor, fragment);
}
