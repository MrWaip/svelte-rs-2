import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const doubled = $.mutable_source(void 0, true);
	let x = $.prop($$props, "x", 9, 1);
	$.legacy_pre_effect(() => $.deep_read_state(x()), () => {
		$.set(doubled, x() * 2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
