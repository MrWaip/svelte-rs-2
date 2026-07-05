App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const s = $.wrap_snippet(App, function($$anchor, a = $.noop, $$arg1) {
	$.validate_snippet_args(...arguments);
	let b = $.derived_safe_equal(() => $.fallback($$arg1?.(), 2));
	$.get(b);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
});
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = 1;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => s($$anchor, () => v), "render", App, 7, 0);
	return $.pop($$exports);
}
