App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{
		id: 1,
		name: "a"
	}, {
		id: 2,
		name: "b"
	}]), "items");
	let duration = 300;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 25, () => items, (item) => item.id, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.animation(div, () => flip, () => ({ duration }));
		$.append($$anchor, div);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
