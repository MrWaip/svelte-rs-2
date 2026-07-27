import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let pending = null;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = ($$anchor) => {
			$.next();
			var text = $.text("wait");
			$.append($$anchor, text);
		};
		$.boundary(node, {
			pending,
			pending
		}, ($$anchor) => {
			$.next();
			var text_1 = $.text("hi");
			$.append($$anchor, text_1);
		});
	}
	$.append($$anchor, fragment);
}
