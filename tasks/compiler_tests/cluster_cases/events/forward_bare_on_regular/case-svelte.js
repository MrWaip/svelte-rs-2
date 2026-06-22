import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	var input = root();
	$.event("click", input, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
	$.append($$anchor, input);
}
