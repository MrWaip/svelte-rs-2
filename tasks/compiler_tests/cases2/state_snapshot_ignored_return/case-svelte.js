App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	function getSnapshot() {
		// svelte-ignore state_snapshot_uncloneable
		return $.snapshot(items, true);
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
