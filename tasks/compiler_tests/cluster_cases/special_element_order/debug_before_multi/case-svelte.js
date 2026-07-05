import * as $ from "svelte/internal/client";
var root = $.from_html(`<span>x</span> <button> </button>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = root();
	$.template_effect(() => {
		console.log({ count: $.snapshot($.get(count)) });
		debugger;
	});
	var button = $.sibling($.first_child(fragment), 2);
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
