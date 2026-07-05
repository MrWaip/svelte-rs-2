import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>+</button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.prop($$props, "count", 12);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.template_effect(() => {
		console.log({ count: $.untrack(() => $.snapshot(count())) });
		debugger;
	});
	$.event("click", button, function click() {
		return $.update_prop(count);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
