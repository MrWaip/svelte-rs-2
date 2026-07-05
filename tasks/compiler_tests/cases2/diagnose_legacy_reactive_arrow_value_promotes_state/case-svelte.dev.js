import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[16, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const handler = $.mutable_source();
	let onInput = $.prop($$props, "onInput", 8, () => {});
	let flag = $.tag($.mutable_source(false), "flag");
	$.legacy_pre_effect(() => ($.deep_read_state(onInput()), $.get(flag)), () => {
		$.set(handler, async (value) => {
			const result = (await $.track_reactivity_loss(onInput()(value, $.get(flag))))();
			if (result) {
				$.set(flag, true);
			}
		});
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	$.event("click", button, function(...$$args) {
		$.apply(() => $.get(handler), this, $$args, App, [16, 18]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
