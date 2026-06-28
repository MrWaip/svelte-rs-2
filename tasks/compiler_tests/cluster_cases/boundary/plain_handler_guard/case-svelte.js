import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handleError(e) {
		console.error(e);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { onerror: handleError }, ($$anchor) => {
		$.next();
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
