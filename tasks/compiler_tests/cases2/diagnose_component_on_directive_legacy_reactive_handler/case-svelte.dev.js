import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const handler = $.mutable_source();
	let cond = $.prop($$props, "cond", 8, false);
	function a() {}
	function b() {}
	$.legacy_pre_effect(() => $.deep_read_state(cond()), () => {
		$.set(handler, cond() ? a : b);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { $$events: { click(...$$args) {
		$.apply(() => $.get(handler), this, $$args, App, [9, 17]);
	} } }), "component", App, 9, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
