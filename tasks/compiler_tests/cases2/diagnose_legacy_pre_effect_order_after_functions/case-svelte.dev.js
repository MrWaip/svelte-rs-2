import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const maxLength = $.mutable_source();
	let step = $.prop($$props, "step", 8);
	function noop() {}
	$.legacy_pre_effect(() => $.deep_read_state(step()), () => {
		$.set(maxLength, step().maxLength ?? Infinity);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	return $.pop($$exports);
}
