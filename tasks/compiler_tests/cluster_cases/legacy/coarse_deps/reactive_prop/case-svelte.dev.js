import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	$.legacy_pre_effect(() => ($.deep_read_state(a()), $.deep_read_state(b())), () => {
		a();
		b();
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
