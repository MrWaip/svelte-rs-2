App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<!><g><path d="M1"></path></g>`, 1), App[$.FILENAME], [[
	6,
	0,
	[[6, 3]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let raw = "<g><circle r={10}/></g>";
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.html(node, () => raw, void 0, true);
	$.next();
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
