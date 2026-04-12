import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>click me</button>`);
export default function App($$anchor) {
	var button = root();
	$.attribute_effect(button, () => ({
		...$$props,
		class: $$props.class ?? ""
	}));
	$.append($$anchor, button);
}
