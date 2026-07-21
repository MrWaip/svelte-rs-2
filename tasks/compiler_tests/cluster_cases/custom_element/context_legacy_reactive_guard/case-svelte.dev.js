import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const y = $.mutable_source();
	let x = 1;
	$.get(y);
	$.legacy_pre_effect(() => {}, () => {
		$.set(y, x * 2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
