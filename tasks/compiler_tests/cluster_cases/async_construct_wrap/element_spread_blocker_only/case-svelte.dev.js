import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function delay(value) {
		return Promise.resolve(value);
	}
	var attrs;
	var $$promises = $.run([async () => attrs = (await $.track_reactivity_loss(delay({ title: "hi" })))()]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({ ...attrs }), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
