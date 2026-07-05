App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { transform } from "./utils.js";
var root = $.add_locations($.from_html(`<div class="output"> </div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 0;
	const fn = $.tag($.derived(() => transform(value)), "fn");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.get(fn)(value)]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
