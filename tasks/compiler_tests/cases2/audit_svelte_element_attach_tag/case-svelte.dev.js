App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	function attachment(node) {}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node_1, () => tag, false, ($$element, $$anchor) => {
			$.attach($$element, () => attachment);
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [6, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
