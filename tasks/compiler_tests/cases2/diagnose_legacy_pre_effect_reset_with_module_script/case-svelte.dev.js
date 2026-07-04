import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export const M = 1;
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8, 0);
	let b = $.prop($$props, "b", 12, "");
	$.legacy_pre_effect(() => ($.deep_read_state(b()), $.deep_read_state(a())), () => {
		b(b() || (a() ? "x" : "y"));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, b()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
