import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button> <!>`, 1);
export default function App($$anchor) {
	let tags = $.state($.proxy(["div", "span"]));
	function bump() {
		$.set(tags, ["p"], true);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 17, () => $.get(tags), $.index, ($$anchor, t) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.element(node_1, () => $.get(t), false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		});
		$.append($$anchor, fragment_1);
	});
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
