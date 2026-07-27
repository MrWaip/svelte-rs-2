import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function h() {
		return 1;
	}
	async function f() {
		return 2;
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(($0, $1) => {
		$.set_attribute(div, "title", `z${$0 ?? ""}`);
		classes = $.set_class(div, 1, "", null, classes, $1);
	}, void 0, [async () => (await $.track_reactivity_loss(h()))(), async () => ({ a: (await $.track_reactivity_loss(f()))() })]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
