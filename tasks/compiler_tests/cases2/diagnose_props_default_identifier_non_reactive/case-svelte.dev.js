App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { noop } from "./helpers";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let onError = $.prop($$props, "onError", 3, noop);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
