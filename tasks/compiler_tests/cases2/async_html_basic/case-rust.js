import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	async function loadContent() {
		return "<b>hello</b>";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [loadContent], (node, $$html) => {
		$.html(node, () => $.get($$html));
	});
	$.append($$anchor, fragment);
}
