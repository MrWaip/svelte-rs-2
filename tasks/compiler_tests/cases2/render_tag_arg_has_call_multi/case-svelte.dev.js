App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const show = $.wrap_snippet(App, function($$anchor, a = $.noop, b = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a() ?? ""} ${b() ?? ""}`));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn1() {
		return "a";
	}
	function fn2() {
		return "b";
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(fn1);
		let $1 = $.derived(fn2);
		$.add_svelte_meta(() => show($$anchor, () => $.get($0), () => $.get($1)), "render", App, 10, 0);
	}
	return $.pop($$exports);
}
