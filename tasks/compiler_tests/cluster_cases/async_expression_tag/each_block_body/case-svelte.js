import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>add</button> <!>`, 1);
export default function App($$anchor) {
	let items = $.proxy([1, 2]);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		$.next();
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [() => delay($.get(item))]);
		$.append($$anchor, text);
	});
	$.delegated("click", button, () => items.push(items.length));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
