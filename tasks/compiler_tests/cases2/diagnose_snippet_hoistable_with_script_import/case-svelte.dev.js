App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { noop } from "./helpers.js";
const socket = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, noop));
	$.append($$anchor, div);
});
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => socket($$anchor), "render", App, 9, 0);
	return $.pop($$exports);
}
