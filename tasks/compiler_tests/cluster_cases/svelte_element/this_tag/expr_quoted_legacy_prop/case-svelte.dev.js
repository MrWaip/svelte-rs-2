import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tag = $.prop($$props, "tag", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(tag);
		$.validate_void_dynamic_element(tag);
		$.element(node, tag, false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		}, void 0, [4, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
