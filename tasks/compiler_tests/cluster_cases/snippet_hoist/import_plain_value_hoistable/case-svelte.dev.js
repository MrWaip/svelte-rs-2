App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { noop } from "./helpers.js";
const foo = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, noop));
	$.append($$anchor, text);
});
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => foo($$anchor), "render", App, 9, 0);
	return $.pop($$exports);
}
