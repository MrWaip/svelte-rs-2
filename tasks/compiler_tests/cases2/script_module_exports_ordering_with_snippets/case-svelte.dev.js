App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor, text = $.noop) {
	$.validate_snippet_args(...arguments);
	var span = root();
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, text()));
	$.append($$anchor, span);
});
export const KIND = "v1";
export function label(name) {
	return `${KIND}:${name}`;
}
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[13, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(() => label($$props.title));
		$.add_svelte_meta(() => row($$anchor, () => $.get($0)), "render", App, 16, 0);
	}
	return $.pop($$exports);
}
