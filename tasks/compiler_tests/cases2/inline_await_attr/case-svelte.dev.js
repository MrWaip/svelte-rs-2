import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var response;
	var $$promises = $.run([async () => response = (await $.track_reactivity_loss(fetch("/api")))()]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(($0) => $.set_attribute(div, "title", $0), void 0, [async () => (await $.track_reactivity_loss(response.text()))()], [$$promises[0]]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
