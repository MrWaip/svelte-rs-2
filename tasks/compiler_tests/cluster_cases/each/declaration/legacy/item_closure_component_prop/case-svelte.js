import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
export default function App($$anchor, $$props) {
	let clicked = $.prop($$props, "clicked", 12);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => ["x"], $.index, ($$anchor, letter) => {
		Widget($$anchor, { handleClick: () => clicked(letter) });
	});
	$.append($$anchor, fragment);
}
