import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let href = $.prop($$props, "href", 8, undefined);
	function getTag() {
		return href() ? "a" : "div";
	}
	function getRole() {
		return href() ? "link" : undefined;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(getTag);
		$.validate_void_dynamic_element(getTag);
		$.element(node, getTag, false, ($$element, $$anchor) => {
			$.attribute_effect($$element, ($0) => ({
				role: $0,
				href: href()
			}), [() => $.untrack(getRole)]);
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [14, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
