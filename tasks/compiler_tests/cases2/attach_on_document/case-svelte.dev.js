App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function track(node) {
		return () => {};
	}
	var $$exports = { ...$.legacy_api() };
	$.attach($.document, () => track);
	return $.pop($$exports);
}
