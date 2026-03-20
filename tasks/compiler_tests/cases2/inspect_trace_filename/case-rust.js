import "svelte/internal/flags/tracing";
App[$.FILENAME] = "src/App.svelte";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function handleClick() {
		return $.trace(() => "handleClick (src/​App.svelte:3:1)", () => {
			$.update(count);
		});
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
