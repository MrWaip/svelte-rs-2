import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="file" multiple=""/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let files = $.prop($$props, "files", 12, undefined);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.bind_files(input, function get() {
		return files();
	}, function set($$value) {
		files($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
