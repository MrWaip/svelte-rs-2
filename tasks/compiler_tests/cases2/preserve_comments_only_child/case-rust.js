import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.append($$anchor, fragment);
}
