import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a, b;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => {
		a = "a";
		b = "b";
	}]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, {
		one: a,
		two: b
	}), void 0, void 0, [$$promises[1], $$promises[1]]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
