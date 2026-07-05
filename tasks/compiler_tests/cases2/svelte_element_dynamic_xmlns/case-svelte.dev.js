App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "rect";
	let ns = "http://www.w3.org/2000/svg";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			$.attribute_effect($$element, () => ({
				xmlns: ns,
				width: "100",
				height: "100"
			}));
		}, () => ns, [6, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
