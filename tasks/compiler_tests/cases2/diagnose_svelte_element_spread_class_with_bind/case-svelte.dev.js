App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let props = $.tag_proxy($.proxy({ id: "bar" }), "props");
	let active = false;
	let el = $.tag($.state(void 0), "el");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			$.bind_this($$element, ($$value) => $.set(el, $$value, true), () => $.get(el));
			$.attribute_effect($$element, () => ({
				...props,
				[$.CLASS]: { active }
			}));
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [8, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
