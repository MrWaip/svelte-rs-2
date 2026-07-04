App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const onClose = () => {};
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "header", { onClose }, null);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
