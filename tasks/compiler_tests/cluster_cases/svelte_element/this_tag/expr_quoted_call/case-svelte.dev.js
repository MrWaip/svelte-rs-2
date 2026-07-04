import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function getTag() {
		return "div";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(getTag);
		$.element(node, getTag, false, void 0, void 0, [2, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
