import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let n = $.prop($$props, "n", 8);
	function run() {
		try {
			const a = 1;
			const b = 2;
			console.log(a, b);
		} catch {}
	}
	if (n()) {
		console.log(n());
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, n()));
	$.delegated("click", button, run);
	$.append($$anchor, button);
}
$.delegate(["click"]);
