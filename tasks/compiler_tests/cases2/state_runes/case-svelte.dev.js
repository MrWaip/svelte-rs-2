App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { debounce } from "es-toolkit";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = "long string value";
	let value2 = undefined;
	let value3 = void 0;
	let value4 = $.tag_proxy($.proxy({}), "value4");
	let value5 = $.tag_proxy($.proxy(value1), "value5");
	let value6 = null;
	let value7 = () => {};
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
