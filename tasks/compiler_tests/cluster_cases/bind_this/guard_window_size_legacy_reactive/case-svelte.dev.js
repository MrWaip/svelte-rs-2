import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const doubled = $.mutable_source();
	let width = $.tag($.mutable_source(0), "width");
	$.legacy_pre_effect(() => $.get(width), () => {
		$.set(doubled, $.get(width) * 2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.bind_window_size("innerWidth", function set($$value) {
		$.set(width, $$value);
	});
	$.append($$anchor, p);
	return $.pop($$exports);
}
