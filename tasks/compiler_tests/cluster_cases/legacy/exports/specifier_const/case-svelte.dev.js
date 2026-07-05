import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const total = 42;
	var $$exports = {
		...$.legacy_api(),
		get total() {
			return total;
		}
	};
	var p = root();
	p.textContent = "42";
	$.append($$anchor, p);
	$.bind_prop($$props, "total", total);
	return $.pop($$exports);
}
