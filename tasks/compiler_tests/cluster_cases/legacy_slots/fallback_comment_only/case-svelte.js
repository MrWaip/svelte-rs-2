import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "a", {}, ($$anchor) => {});
	$.append($$anchor, fragment);
}
