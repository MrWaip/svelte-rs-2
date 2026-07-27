import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	const row = ($$anchor) => {
		$.next();
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [() => delay($.get(x))]);
		$.append($$anchor, text);
	};
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment_1 = root();
	var button = $.first_child(fragment_1);
	var node = $.sibling(button, 2);
	row(node);
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment_1);
}
$.delegate(["click"]);
