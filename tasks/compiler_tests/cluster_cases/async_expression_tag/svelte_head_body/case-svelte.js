import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button>`);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var button = root();
	$.head("q2w0q4", ($$anchor) => {
		var text = $.text();
		$.template_effect(($0) => $.set_text(text, $0), void 0, [() => delay($.get(x))]);
		$.append($$anchor, text);
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, button);
}
$.delegate(["click"]);
