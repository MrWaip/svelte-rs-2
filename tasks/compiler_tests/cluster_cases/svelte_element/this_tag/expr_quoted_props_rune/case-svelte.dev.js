App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => $$props.tag);
		$.validate_void_dynamic_element(() => $$props.tag);
		$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		}, void 0, [4, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
