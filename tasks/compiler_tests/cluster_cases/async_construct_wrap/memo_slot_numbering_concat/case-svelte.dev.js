import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>y</div>`), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, ($0, $1) => ({
		...{},
		title: `a${$1 ?? ""}b`,
		id: $0
	}), [() => fn()], [async () => (await $.track_reactivity_loss("x"))()]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
