import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const tag = $.mutable_source();
	let n = $.prop($$props, "n", 8);
	$.legacy_pre_effect(() => $.deep_read_state(n()), () => {
		$.set(tag, "h" + n());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => $.get(tag));
		$.validate_void_dynamic_element(() => $.get(tag));
		$.element(node, () => $.get(tag), false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		}, void 0, [5, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
