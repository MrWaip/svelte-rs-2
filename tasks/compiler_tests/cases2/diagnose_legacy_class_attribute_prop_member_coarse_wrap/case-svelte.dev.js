import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let config = $.prop($$props, "config", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx(($.deep_read_state(config()), $.untrack(() => config().cls)))));
	$.append($$anchor, div);
	return $.pop($$exports);
}
