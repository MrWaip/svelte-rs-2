import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const visible = $.mutable_source(void 0, true);
	let n = $.prop($$props, "n", 9);
	let cache = $.tag($.mutable_source({}, true), "cache");
	function bump(i) {
		cache[i] = i;
	}
	$.legacy_pre_effect(() => (cache, $.deep_read_state(n())), () => {
		$.set(visible, compute(cache, n()));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(visible)));
	$.event("click", button, function click() {
		return bump(1);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
