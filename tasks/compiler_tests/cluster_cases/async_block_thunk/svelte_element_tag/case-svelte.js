import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [async () => (await $.save(delay($.get(x))))() + "div"], (node, $$tag) => {
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			var text = $.text("content");
			$.append($$anchor, text);
		});
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
