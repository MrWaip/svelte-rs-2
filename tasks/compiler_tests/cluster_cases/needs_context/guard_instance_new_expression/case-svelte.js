import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let n = $.state(0);
	function make() {
		return new Date();
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(n)));
	$.delegated("click", button, () => {
		$.update(n);
		make();
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
