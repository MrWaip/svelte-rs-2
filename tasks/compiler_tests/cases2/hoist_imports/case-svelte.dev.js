App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { tree } from "./tree.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
