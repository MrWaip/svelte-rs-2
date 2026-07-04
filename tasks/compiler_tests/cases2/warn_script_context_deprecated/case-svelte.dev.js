App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export let value = 1;
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
