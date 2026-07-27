import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.first_child(fragment);
	$.each(node, 0, () => ({ length: 1 }), $.index, ($$anchor, $$item) => {
		const data = $.derived_safe_equal(() => 1);
		$.next();
		var text = $.text();
		text.nodeValue = $.get(data);
		$.append($$anchor, text);
	});
	var node_1 = $.sibling(node, 2);
	$.each(node_1, 0, () => ({ length: 0 }), $.index, ($$anchor, $$item) => {
		$.next();
		var text_1 = $.text("x");
		$.append($$anchor, text_1);
	}, ($$anchor) => {
		const data = $.derived_safe_equal(() => 2);
		$.next();
		var text_2 = $.text();
		text_2.nodeValue = $.get(data);
		$.append($$anchor, text_2);
	});
	$.append($$anchor, fragment);
}
