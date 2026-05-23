import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let onClick = $.prop($$props, "onClick", 8);
	$.event("click", $.document.body, $.preventDefault(function(...$$args) {
		onClick()?.apply(this, $$args);
	}));
}
