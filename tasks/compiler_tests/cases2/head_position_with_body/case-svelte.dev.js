App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = "Page";
	function handleClick() {}
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.head("q2w0q4", ($$anchor) => {
		$.effect(() => {
			$.document.title = "Page";
		});
	});
	$.event("click", $.document.body, handleClick);
	p.textContent = "Content: Page";
	$.append($$anchor, p);
	return $.pop($$exports);
}
