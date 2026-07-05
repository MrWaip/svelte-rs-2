App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let content = "<p>safe</p>";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.html(node, () => content, void 0, void 0, void 0, true);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
