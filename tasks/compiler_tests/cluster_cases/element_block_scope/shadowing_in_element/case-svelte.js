import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button> <div><b></b></div>`, 1);
export default function App($$anchor) {
	let x = $.state(1);
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var div = $.sibling(button, 2);
	{
		const x = "inner";
		var b = $.child(div);
		b.textContent = "inner";
		$.reset(div);
	}
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
