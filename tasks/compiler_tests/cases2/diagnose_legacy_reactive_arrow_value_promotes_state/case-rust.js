import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const handler = $.mutable_source();
	let onInput = $.prop($$props, "onInput", 8, () => {});
	let flag = $.mutable_source(false);
	$.legacy_pre_effect(() => ($.deep_read_state(onInput()), $.get(flag)), () => {
		$.set(handler, async (value) => {
			const result = await onInput()(value, $.get(flag));
			if (result) {
				$.set(flag, true);
			}
		});
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	$.event("click", button, function(...$$args) {
		$.get(handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
}
