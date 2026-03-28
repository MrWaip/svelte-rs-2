import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	async function getTag() {
		return "div";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => await getTag(), false, ($$element, $$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
