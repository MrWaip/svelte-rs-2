import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span></span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => $.untrack(() => new Array(4).fill(null)), $.index, ($$anchor, _, i) => {
		var span = root();
		span.textContent = i;
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
	$.pop();
}
