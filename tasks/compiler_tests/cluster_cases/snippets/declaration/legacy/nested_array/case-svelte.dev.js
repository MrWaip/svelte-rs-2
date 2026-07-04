import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const s = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	var $$array = $.derived(() => $.to_array($$arg0?.(), 2));
	var $$array_1 = $.derived(() => $.to_array($$array[0], 2));
	var $$array_2 = $.derived(() => $.to_array($$array[1], 2));
	let a = () => $.get($$array_1)[0];
	a();
	let b = () => $.get($$array_1)[1];
	b();
	let c = () => $.get($$array_2)[0];
	c();
	let d = () => $.get($$array_2)[1];
	d();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}${d() ?? ""}`));
	$.append($$anchor, button);
});
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let v = [[1, 2], [3, 4]];
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => s($$anchor, () => v), "render", App, 8, 0);
	return $.pop($$exports);
}
