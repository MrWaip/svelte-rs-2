import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<button> </button> <!>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var node = $.sibling(button, 2);
	{
		let $0 = $.derived(() => !$.get(count));
		Child(node, {
			get "aria-disabled"() {
				return $.get($0);
			},
			children: ($$anchor, $$slotProps) => {
				$.next();
				var text_1 = $.text("hi");
				$.append($$anchor, text_1);
			},
			$$slots: { default: true }
		});
	}
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
