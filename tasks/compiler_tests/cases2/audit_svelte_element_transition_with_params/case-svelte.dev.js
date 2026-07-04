App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = "div";
	let duration = 300;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => tag);
		$.validate_void_dynamic_element(() => tag);
		$.element(node, () => tag, false, ($$element, $$anchor) => {
			$.transition(3, $$element, () => fade, () => ({ duration }));
			var text = $.text("x");
			$.append($$anchor, text);
		}, void 0, [7, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
