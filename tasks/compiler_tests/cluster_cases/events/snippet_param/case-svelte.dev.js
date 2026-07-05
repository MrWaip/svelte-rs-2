App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor, handler = $.noop) {
	$.validate_snippet_args(...arguments);
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.apply(handler, this, $$args, App, [2, 18]);
	});
	$.append($$anchor, button);
});
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => row($$anchor, () => () => {}), "render", App, 5, 0);
	return $.pop($$exports);
}
$.delegate(["click"]);
