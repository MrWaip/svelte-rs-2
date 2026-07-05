App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const summary = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var section = root();
	$.append($$anchor, section);
});
var root = $.add_locations($.from_html(`<section class="summary svelte-ic1cb7">summary</section>`), App[$.FILENAME], [[12, 4]]);
var root_1 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let active = true;
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	$.set_class(div, 1, "chunk-shell svelte-ic1cb7", null, {}, { state: active });
	var node = $.child(div);
	$.add_svelte_meta(() => summary(node), "render", App, 16, 4);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
