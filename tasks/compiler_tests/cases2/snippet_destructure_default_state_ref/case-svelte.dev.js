App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor, $$arg0) {
		$.validate_snippet_args(...arguments);
		let values = $.derived_safe_equal(() => $.fallback(($$arg0?.()).values, () => [counter], true));
		$.get(values);
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(values).length));
		$.append($$anchor, span);
	});
	let counter = 0;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => row($$anchor, () => ({})), "render", App, 9, 0);
	return $.pop($$exports);
}
