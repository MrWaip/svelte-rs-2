App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!foo>bar</!foo>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var _foo = root();
	$.append($$anchor, _foo);
	return $.pop($$exports);
}
