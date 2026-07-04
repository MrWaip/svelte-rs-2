App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let val = 0;
	$.inspect(() => [val], (...$$args) => console.warn(...$$args));
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
