App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const pair = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var div = root();
	$.append($$anchor, div);
});
var root = $.add_locations($.from_html(`<div class="after svelte-1hn6tgg">after</div>`), App[$.FILENAME], [[8, 4]]);
var root_1 = $.add_locations($.from_html(`<span class="before svelte-1hn6tgg">before</span> <!> <div>other</div>`, 1), App[$.FILENAME], [[5, 0], [13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	$.add_svelte_meta(() => pair(node), "render", App, 11, 0);
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
