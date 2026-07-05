App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([[[1, 2], [3, 4]]]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		var $$array = $.derived(() => $.to_array($.get($$item), 2));
		var $$array_1 = $.derived(() => $.to_array($.get($$array)[0], 2));
		var $$array_2 = $.derived(() => $.to_array($.get($$array)[1], 2));
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
	}), "each", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
