import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { A } from "./a";
import { B } from "./b";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const y = $.mutable_source();
	let x = $.prop($$props, "x", 8);
	$.legacy_pre_effect(() => ($.deep_read_state(x()), B, A), () => {
		$.set(y, (function() {
			switch (x()) {
				case A.ONE: return B;
				default: return "";
			}
		})());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(y)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
