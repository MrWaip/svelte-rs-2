import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> `, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve({ value });
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(($0) => $.set_text(text, ` ${$0 ?? ""}`), void 0, [async () => (await $.save(delay($.get(x))))().value]);
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
