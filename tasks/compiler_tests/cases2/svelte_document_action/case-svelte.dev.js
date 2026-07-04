App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import tooltip from "./tooltip.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.action($.document, ($$node) => tooltip?.($$node));
	return $.pop($$exports);
}
