import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div>2</div> <div>1</div>`, 1);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, $$item) => {
		var fragment_1 = root_1();
		$.next(2);
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
