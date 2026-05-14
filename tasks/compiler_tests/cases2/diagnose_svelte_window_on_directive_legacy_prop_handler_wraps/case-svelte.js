import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let closeModal = $.prop($$props, "closeModal", 8);
	$.event("close", $.window, $.preventDefault(function(...$$args) {
		closeModal()?.apply(this, $$args);
	}));
}
