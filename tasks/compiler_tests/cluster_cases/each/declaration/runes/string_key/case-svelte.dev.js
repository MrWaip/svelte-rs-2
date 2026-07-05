App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{
		"a-b": 1,
		"c d": 2
	}]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let ab = () => $.get($$item)["a-b"];
		ab();
		let cd = () => $.get($$item)["c d"];
		cd();
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${ab() ?? ""}${cd() ?? ""}`));
		$.append($$anchor, button);
	}), "each", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
