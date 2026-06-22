import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.event("resize", $.window, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
}
