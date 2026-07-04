App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>content</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let isActive = false;
	const bold = true;
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_class(div, 1, $.clsx({
		active: isActive,
		bold
	}));
	$.append($$anchor, div);
	return $.pop($$exports);
}
