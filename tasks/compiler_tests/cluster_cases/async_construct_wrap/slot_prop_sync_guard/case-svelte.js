import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor, $$props) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		let $0 = $.derived(() => delay($.get(x)));
		$.slot(node, $$props, "default", { get value() {
			return $.get($0);
		} }, null);
	}
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
