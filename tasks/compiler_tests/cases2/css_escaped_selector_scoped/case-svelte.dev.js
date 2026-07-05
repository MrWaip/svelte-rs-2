App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="foo:bar svelte-os1qct">class</div> <div id="hero:id" class="svelte-os1qct">id</div> <div class="miss">outside</div>`, 1), App[$.FILENAME], [
	[6, 0],
	[7, 0],
	[8, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
