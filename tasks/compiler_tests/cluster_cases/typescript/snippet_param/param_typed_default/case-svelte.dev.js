App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let c = $.derived_safe_equal(() => $.fallback($$arg0?.(), 4));
	$.get(c);
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(c)));
	$.append($$anchor, span);
});
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<button> </button> <!>`, 1), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var text_1 = $.child(button, true);
	$.reset(button);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => row(node, () => $.get(count)), "render", App, 8, 0);
	$.template_effect(() => $.set_text(text_1, $.get(count)));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
