App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "button";
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			var event_handler = () => $.update(count);
			$.attribute_effect($$element, () => ({ onclick: event_handler }));
			var text = $.text("Click");
			$.append($$anchor, text);
		}, void 0, [6, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
