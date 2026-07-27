import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => a = "a"]);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.template_effect(() => input.defaultValue = a, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, input);
	return $.pop($$exports);
}
