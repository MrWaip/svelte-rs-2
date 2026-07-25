import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const tag = $.mutable_source();
	let n = $.prop($$props, "n", 8);
	$.legacy_pre_effect(() => $.deep_read_state(n()), () => {
		$.set(tag, "h" + n());
	});
	$.legacy_pre_effect_reset();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => $.get(tag), false, ($$element, $$anchor) => {
		var text = $.text("hello");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
}
