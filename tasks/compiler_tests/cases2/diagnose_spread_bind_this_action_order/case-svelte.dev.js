App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { act } from "./act";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let ref;
	let attrs = {};
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, () => ({ ...attrs }));
	$.action(div, ($$node) => act?.($$node));
	$.bind_this(div, ($$value) => ref = $$value, () => ref);
	$.append($$anchor, div);
	return $.pop($$exports);
}
