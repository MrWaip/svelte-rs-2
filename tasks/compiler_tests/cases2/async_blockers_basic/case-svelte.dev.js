import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 1;
	var data, y;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))(), () => y = data.value]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, y), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
