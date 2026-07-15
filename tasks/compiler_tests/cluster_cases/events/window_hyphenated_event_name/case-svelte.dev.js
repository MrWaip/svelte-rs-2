App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function flush() {}
	var $$exports = { ...$.legacy_api() };
	$.event("obank-tab-stop", $.window, function obank_tab_stop() {
		return flush();
	});
	return $.pop($$exports);
}
