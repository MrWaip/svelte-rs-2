import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let size = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => `h${size}`);
		$.element(node, () => `h${size}`, false, void 0, void 0, [2, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
