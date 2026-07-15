import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span data-xxx="&amp;copy=value" style="&amp;copy=value"></span> <span data-xxx="©" style="©"></span> <span data-xxx="©=value" style="©=value"></span> <span data-xxx="&amp;copyotherstring=value" style="&amp;copyotherstring=value"></span> <span data-xxx="&amp;copy123=value" style="&amp;copy123=value"></span> <span data-xxx="&amp;rect=value" style="&amp;rect=value"></span> <span data-xxx="▭=value" style="▭=value"></span>`, 1), App[$.FILENAME], [
	[1, 0],
	[2, 0],
	[3, 0],
	[4, 0],
	[5, 0],
	[6, 0],
	[7, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(12);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
