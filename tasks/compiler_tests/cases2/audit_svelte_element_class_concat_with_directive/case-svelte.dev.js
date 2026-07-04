App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let b = $.tag($.state("two"), "b");
	let active = false;
	$.user_effect(() => {
		$.set(b, $.get(b) + "");
	});
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			$.attribute_effect($$element, () => ({
				class: `one ${$.get(b) ?? ""}`,
				[$.CLASS]: { active }
			}));
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [8, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
