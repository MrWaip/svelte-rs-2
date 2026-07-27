App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ x: null }), "obj");
	let src = $.tag_proxy($.proxy({}), "src");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => "button");
		$.validate_void_dynamic_element(() => "button");
		$.element(node, () => "button", false, ($$element, $$anchor) => {
			var event_handler = () => obj.x = src;
			$.attribute_effect($$element, () => ({ onclick: event_handler }));
			var text = $.text("go");
			$.append($$anchor, text);
		}, void 0, [6, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
