App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const badge = $.wrap_snippet(App, function($$anchor, text = $.noop) {
		$.validate_snippet_args(...arguments);
		var span = root();
		$.set_class(span, 1, "badge svelte-yczv4j", null, {}, { primary: $.strict_equals(variant, "primary") });
		var text_1 = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text_1, text()));
		$.append($$anchor, span);
	});
	let variant = "primary";
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => badge($$anchor, () => "hi"), "render", App, 9, 0);
	return $.pop($$exports);
}
