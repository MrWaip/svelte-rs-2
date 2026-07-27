import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button> <!>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	function bump() {
		$.update(n);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.element(node, () => "h", false, ($$element, $$anchor) => {
		var text = $.text("hello");
		$.append($$anchor, text);
	});
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
