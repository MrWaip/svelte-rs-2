import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<b>fallback</b>`);
var root_1 = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor, $$props) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, void 0, [() => delay($.get(x))], (node, $0) => {
		$.slot(node, $$props, "default", { get value() {
			return $.get($0);
		} }, ($$anchor) => {
			var b = root();
			$.append($$anchor, b);
		});
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
