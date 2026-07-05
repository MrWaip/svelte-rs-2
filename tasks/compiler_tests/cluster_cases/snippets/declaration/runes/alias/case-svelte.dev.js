App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const s = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let x = () => ($$arg0?.()).a;
	x();
	let y = () => ($$arg0?.()).b;
	y();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${x() ?? ""}${y() ?? ""}`));
	$.append($$anchor, button);
});
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag_proxy($.proxy({
		a: 1,
		b: 2
	}), "v");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => s($$anchor, () => v), "render", App, 7, 0);
	return $.pop($$exports);
}
