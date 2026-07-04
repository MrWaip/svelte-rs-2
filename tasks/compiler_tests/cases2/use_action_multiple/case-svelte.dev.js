App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { focus, tooltip } from "./actions.js";
var root = $.add_locations($.from_html(`<div>text</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let config = { text: "hello" };
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.action(div, ($$node) => focus?.($$node));
	$.action(div, ($$node, $$action_arg) => tooltip?.($$node, $$action_arg), () => config);
	$.append($$anchor, div);
	return $.pop($$exports);
}
