App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let active = false;
	function action(node) {}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node_1, () => tag, false, ($$element, $$anchor) => {
			$.action($$element, ($$node) => action?.($$node));
			$.set_class($$element, 0, "", null, {}, { active });
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [7, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
