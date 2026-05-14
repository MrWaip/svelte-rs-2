import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
export default function App($$anchor) {
	let tag = "div";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false, ($$element, $$anchor) => {
		$.transition(3, $$element, () => fade);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
