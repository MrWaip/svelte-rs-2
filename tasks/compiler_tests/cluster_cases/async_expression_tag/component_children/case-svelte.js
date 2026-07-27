import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	Child(node, {
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text();
			$.template_effect(($0) => $.set_text(text, $0), void 0, [() => delay($.get(x))]);
			$.append($$anchor, text);
		},
		$$slots: { default: true }
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
