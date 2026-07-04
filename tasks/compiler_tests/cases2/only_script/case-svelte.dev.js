App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let i = 10;
	i++;
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
