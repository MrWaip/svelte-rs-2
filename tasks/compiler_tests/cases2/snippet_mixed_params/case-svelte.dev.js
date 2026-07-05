App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor, label = $.noop, $$arg1, $$arg2) {
	$.validate_snippet_args(...arguments);
	let id = () => ($$arg1?.()).id;
	id();
	var $$array = $.derived(() => $.to_array($$arg2?.(), 1));
	let value = () => $.get($$array)[0];
	value();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${label() ?? ""}: ${id() ?? ""} = ${value() ?? ""}`));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{ id: 1 }]), "items");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => row($$anchor, () => "test", () => items[0], () => [42]), "render", App, 9, 0);
	return $.pop($$exports);
}
