import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let onclick = $.prop($$props, "onclick", 8, undefined);
	let useFn = $.prop($$props, "useFn", 8, undefined);
	let useArgs = $.prop($$props, "useArgs", 24, () => []);
	let href = $.prop($$props, "href", 8, undefined);
	function getTag() {
		return href() ? "a" : "div";
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(getTag);
		$.validate_void_dynamic_element(getTag);
		$.element(node, getTag, false, ($$element, $$anchor) => {
			$.action($$element, ($$node, $$action_arg) => useFn()?.($$node, $$action_arg), () => useArgs() || []);
			$.attribute_effect($$element, () => ({ href: href() }));
			$.event("click", $$element, function(...$$args) {
				$.apply(onclick, this, $$args, App, [18, 14]);
			});
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [14, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
