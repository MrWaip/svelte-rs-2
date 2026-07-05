import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 24, () => []);
	let prefix = $.prop($$props, "prefix", 8, "");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 3, items, (item, i) => prefix() + i, ($$anchor, item) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, p);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
