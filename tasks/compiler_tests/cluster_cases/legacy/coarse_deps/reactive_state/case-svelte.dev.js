import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let s = $.tag($.mutable_source(0), "s");
	function inc() {
		$.update(s);
	}
	$.legacy_pre_effect(() => ($.deep_read_state(a()), $.get(s)), () => {
		a();
		$.get(s);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
