import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let store = $.prop($$props, "store", 8, undefined);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	$.event("click", button, function(...$$args) {
		$.apply(() => store()?.reset, this, $$args, App, [4, 18]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
