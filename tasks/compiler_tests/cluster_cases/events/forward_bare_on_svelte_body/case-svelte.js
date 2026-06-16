import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.event("click", $.document.body, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
}
