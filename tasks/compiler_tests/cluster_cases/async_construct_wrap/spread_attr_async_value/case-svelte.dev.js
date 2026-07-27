import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>neato</p>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.attribute_effect(p, ($0) => ({
		...{},
		class: $0
	}), void 0, [async () => (await $.track_reactivity_loss("neato"))()]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
