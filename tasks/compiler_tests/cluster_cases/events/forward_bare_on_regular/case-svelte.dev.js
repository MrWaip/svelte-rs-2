import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.event("click", input, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
