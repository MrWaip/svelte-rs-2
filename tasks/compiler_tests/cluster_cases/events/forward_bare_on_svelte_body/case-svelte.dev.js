import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.event("click", $.document.body, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
	return $.pop($$exports);
}
