import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	async function getValue() {
		return 42;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [getValue], (node, $$key) => {
		$.key(node, () => $.get($$key), ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		});
	});
	$.append($$anchor, fragment);
}
