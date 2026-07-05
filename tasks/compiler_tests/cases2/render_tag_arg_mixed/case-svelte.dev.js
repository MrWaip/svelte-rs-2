App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const show = $.wrap_snippet(App, function($$anchor, greeting = $.noop, person = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${greeting() ?? ""} ${person() ?? ""}`));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = "world";
	function greet() {
		return "hello";
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(greet);
		$.add_svelte_meta(() => show($$anchor, () => $.get($0), () => name), "render", App, 10, 0);
	}
	return $.pop($$exports);
}
