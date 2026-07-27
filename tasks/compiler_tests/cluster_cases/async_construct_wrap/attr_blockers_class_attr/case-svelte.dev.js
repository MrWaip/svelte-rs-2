import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var value;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => value = "value"]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx(value)), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
