import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { helper } from "./helper.js";
var root = $.from_html(`<p>ok</p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	var $$exports = { helper };
	var p = root();
	$.append($$anchor, p);
	$.bind_prop($$props, "helper", helper);
	return $.pop($$exports);
}
