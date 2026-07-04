import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>+</button>`);
export default function App($$anchor) {
	let count = $.mutable_source(0);
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	var button = root();
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(() => {
			$.document.title = `Count: ${$.get(count) ?? ""}`;
		});
	});
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
