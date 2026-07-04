App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="svelte-1mj6a7z">inside</div> <div data-state="closed">outside</div>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let open = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "data-state", open ? "open" : "closed");
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
