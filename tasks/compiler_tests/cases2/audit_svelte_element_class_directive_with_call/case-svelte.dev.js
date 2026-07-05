App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let n = 0;
	function compute(x) {
		return x > 0;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			let classes;
			$.template_effect(($0) => classes = $.set_class($$element, 0, "", null, classes, $0), [() => ({ active: compute(n) })]);
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [7, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
