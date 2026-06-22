import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function getTag() {
		return "div";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, getTag, false);
	$.append($$anchor, fragment);
}
