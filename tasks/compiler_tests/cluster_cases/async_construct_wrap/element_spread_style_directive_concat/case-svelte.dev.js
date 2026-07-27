import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function g() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...{ q: 1 },
		[$.STYLE]: $0
	}), void 0, [async () => ({ color: `${(await $.track_reactivity_loss(g()))() ?? ""}px` })]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
