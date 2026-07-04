App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { slide } from "svelte/transition";
var root = $.add_locations($.from_html(`<div>hello</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.transition(2, div, () => slide);
	$.append($$anchor, div);
	return $.pop($$exports);
}
