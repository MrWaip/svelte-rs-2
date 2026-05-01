App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const greeting = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let label = () => $$arg0?.().label;
	label();
	let name = $.derived_safe_equal(() => $.fallback($$arg0?.().name, "world"));
	$.get(name);
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${label() ?? ""}: ${$.get(name) ?? ""}`));
	$.append($$anchor, p);
});
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => greeting($$anchor, () => ({ label: "Hi" })), "render", App, 5, 0);
	return $.pop($$exports);
}
