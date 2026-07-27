import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	async function getTag() {
		return "div";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [getTag], (node, $$tag) => {
		$.element(node, () => $.get($$tag), false);
	});
	$.append($$anchor, fragment);
}
