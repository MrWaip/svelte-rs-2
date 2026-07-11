import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const visible = $.mutable_source(void 0, true);
	let n = $.prop($$props, "n", 9);
	let total = $.tag($.mutable_source(0, true), "total");
	function bump() {
		$.set(total, $.get(total) + 1);
	}
	$.legacy_pre_effect(() => ($.get(total), $.deep_read_state(n())), () => {
		$.set(visible, compute($.get(total), n()));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(visible)));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
