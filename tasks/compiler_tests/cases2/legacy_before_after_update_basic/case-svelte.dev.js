import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { beforeUpdate, afterUpdate } from "svelte";
var root = $.add_locations($.from_html(`<p>hooks</p>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	beforeUpdate(() => {
		console.log("before");
	});
	afterUpdate(() => {
		console.log("after");
	});
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	$.append($$anchor, p);
	return $.pop($$exports);
}
