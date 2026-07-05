App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fly, fade } from "svelte/transition";
var root = $.add_locations($.from_html(`<div>hello</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.transition(1, div, () => fly, () => ({ y: 200 }));
	$.transition(2, div, () => fade);
	$.append($$anchor, div);
	return $.pop($$exports);
}
