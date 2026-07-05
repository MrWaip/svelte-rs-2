import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	async function getPromise() {
		return fetch("/api");
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [], (node) => {
		$.await(node, getPromise, null, ($$anchor, value) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(value)));
			$.append($$anchor, p);
		});
	});
	$.append($$anchor, fragment);
}
