import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => "div");
		$.element(node, () => "div", false, ($$element, $$anchor) => {
			$.event("click", $$element, function($$arg) {
				$.bubble_event.call(this, $$props, $$arg);
			});
		}, void 0, [1, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
