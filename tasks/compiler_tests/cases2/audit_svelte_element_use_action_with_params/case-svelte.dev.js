App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	function action(node, opts) {}
	let opts = $.tag_proxy($.proxy({ x: 1 }), "opts");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node_1, () => tag, false, ($$element, $$anchor) => {
			$.action($$element, ($$node, $$action_arg) => action?.($$node, $$action_arg), () => opts);
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [7, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
