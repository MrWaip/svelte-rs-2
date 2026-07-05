App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const show = $.wrap_snippet(App, function($$anchor, data = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, data()));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(getData);
		$.add_svelte_meta(() => show($$anchor, () => $.get($0)), "render", App, 9, 0);
	}
	return $.pop($$exports);
}
