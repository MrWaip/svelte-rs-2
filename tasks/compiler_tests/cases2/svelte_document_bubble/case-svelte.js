import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.event("keydown", $.document, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
}
