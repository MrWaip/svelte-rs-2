import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>y</div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(($0) => classes = $.set_class(div, 1, $0, null, classes, { b: true }), void 0, [async () => $.clsx((await $.track_reactivity_loss("a"))())]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
