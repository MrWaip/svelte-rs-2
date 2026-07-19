App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { noop } from "./x.js";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	noop(count);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, count));
	$.delegated("click", button, function click() {
		return count++;
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
