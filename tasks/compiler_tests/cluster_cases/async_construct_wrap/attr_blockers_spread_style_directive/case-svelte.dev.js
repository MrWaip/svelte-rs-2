import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var color;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => color = "red"]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({
		...$$props.rest,
		[$.STYLE]: { color }
	}), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
