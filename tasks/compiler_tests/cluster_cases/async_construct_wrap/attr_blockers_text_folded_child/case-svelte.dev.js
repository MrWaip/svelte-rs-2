import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var msg;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => msg = "hi"]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, "hi"), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
