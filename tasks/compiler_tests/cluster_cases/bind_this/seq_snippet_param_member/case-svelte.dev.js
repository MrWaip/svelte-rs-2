App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const funBind = $.wrap_snippet(App, function($$anchor, context = $.noop) {
	$.validate_snippet_args(...arguments);
	var input = root();
	$.bind_this(input, (e) => context().element = e, () => {});
	$.append($$anchor, input);
});
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[4, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => funBind($$anchor, () => ({ set element(e) {} })), "render", App, 6, 0);
	return $.pop($$exports);
}
