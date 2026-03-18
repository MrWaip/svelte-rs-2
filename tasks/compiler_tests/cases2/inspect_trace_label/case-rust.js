import "svelte/internal/flags/tracing";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.state(0);
	function handleClick() {
		return $.trace(() => "custom label", () => {
			$.update(count);
		});
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
