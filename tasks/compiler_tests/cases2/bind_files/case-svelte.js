import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="file"/>`);
export default function App($$anchor) {
	let files = $.state(void 0);
	var input = root();
	$.bind_files(input, () => $.get(files), ($$value) => $.set(files, $$value));
	$.append($$anchor, input);
}
