import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="file" multiple=""/>`);
export default function App($$anchor, $$props) {
	let files = $.prop($$props, "files", 12, undefined);
	var input = root();
	$.bind_files(input, files);
	$.append($$anchor, input);
}
