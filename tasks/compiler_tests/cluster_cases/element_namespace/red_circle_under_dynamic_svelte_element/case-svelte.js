import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<circle r="5"></circle>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => "svg", false, ($$element, $$anchor) => {
		var circle = root_1();
		$.append($$anchor, circle);
	});
	$.append($$anchor, fragment);
}
