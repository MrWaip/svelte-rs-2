import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const s = $.wrap_snippet(App, function($$anchor, $$arg0) {
	$.validate_snippet_args(...arguments);
	let a = () => ($$arg0?.()).a;
	a();
	let rest = () => $.exclude_from_object($$arg0?.(), ["a"]);
	rest();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${(rest(), $.untrack(() => rest().b)) ?? ""}`));
	$.append($$anchor, button);
});
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let v = {
		a: 1,
		b: 2,
		c: 3
	};
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => s($$anchor, () => v), "render", App, 8, 0);
	return $.pop($$exports);
}
