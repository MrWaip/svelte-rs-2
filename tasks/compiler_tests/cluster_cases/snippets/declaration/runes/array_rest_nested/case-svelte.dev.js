App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const s = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	var $$array = $.derived(() => $.to_array($$arg0?.()));
	var $$array_1 = $.derived(() => $.to_array($$array.slice(1), 2));
	let a = () => $.get($$array)[0];
	a();
	let b = () => $.get($$array_1)[0];
	b();
	let c = () => $.get($$array_1)[1];
	c();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}`));
	$.append($$anchor, button);
});
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "v");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => s($$anchor, () => v), "render", App, 7, 0);
	return $.pop($$exports);
}
