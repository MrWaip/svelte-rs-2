import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { helper } from "./helper.js";
var root = $.add_locations($.from_html(`<p>ok</p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = {
		...$.legacy_api(),
		get helper() {
			return helper;
		}
	};
	var p = root();
	$.append($$anchor, p);
	$.bind_prop($$props, "helper", helper);
	return $.pop($$exports);
}
