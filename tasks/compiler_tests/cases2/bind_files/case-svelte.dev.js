App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="file"/>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let files = $.tag($.state(void 0), "files");
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.bind_files(input, function get() {
		return $.get(files);
	}, function set($$value) {
		$.set(files, $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
