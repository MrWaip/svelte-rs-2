App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const foo = $.wrap_snippet(App, function($$anchor, a = $.noop, b = $.noop) {
	$.validate_snippet_args(...arguments);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `Hello world ${a() + b()}`));
	$.append($$anchor, text);
});
export { foo };
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
