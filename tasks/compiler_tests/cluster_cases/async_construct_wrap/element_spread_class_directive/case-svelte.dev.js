import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <div></div>`, 1), App[$.FILENAME], [[9, 0], [11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	$.attribute_effect(div, ($0) => ({
		...$0,
		[$.CLASS]: { on: $.get(x) > 0 }
	}), void 0, [async () => (await $.track_reactivity_loss(delay({ title: $.get(x) })))()]);
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
