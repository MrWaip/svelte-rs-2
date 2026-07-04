App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { api } from "./api.js";
var root = $.add_locations($.from_html(`<button>click</button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function doSomething() {
		api.call();
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, doSomething);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
