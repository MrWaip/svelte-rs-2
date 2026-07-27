import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8, 42);
	let a = $.tag($.mutable_source(), "a");
	let b = $.tag($.mutable_source(), "b");
	$.legacy_pre_effect(() => $.deep_read_state(x()), () => {
		$.set(b, (function(a) {
			return a;
		})(x()));
	});
	$.legacy_pre_effect(() => $.get(b), () => {
		$.set(a, $.get(b));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${$.get(b) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
