import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, item) => {
		$.next();
		var text = $.text();
		text.nodeValue = constValue;
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
