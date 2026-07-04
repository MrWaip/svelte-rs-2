import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var response;
	var $$promises = $.run([async () => response = (await $.track_reactivity_loss(fetch("/api")))()]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `Hello ${$0 ?? ""}!`), void 0, [async () => (await $.track_reactivity_loss(response.text()))()], [$$promises[0]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
