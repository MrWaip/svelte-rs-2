import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let prop = $.prop($$props, "prop", 8);
	let local = $.tag($.mutable_source(0), "local");
	let out = $.tag($.mutable_source(0), "out");
	$.legacy_pre_effect(() => ($.get(local), $.deep_read_state(prop())), () => {
		if (true) {
			$.set(local, 1);
			$.set(out, (prop() || 0) + $.get(local));
		}
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(out)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
