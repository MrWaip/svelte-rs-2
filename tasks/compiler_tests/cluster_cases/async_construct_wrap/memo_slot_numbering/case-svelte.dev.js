import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>x</p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.attribute_effect(p, ($0, $1) => ({
		...{},
		class: $1,
		id: $0
	}), [() => fn()], [async () => (await $.track_reactivity_loss("neato"))()]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
