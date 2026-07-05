App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"items"
]);
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.prop($$props, "items", 19, () => []), rest = $.rest_props($$props, rest_excludes, "rest");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 27, items, (item) => item.id, ($$anchor, item, idx) => {
		var p = root();
		$.attribute_effect(p, () => ({
			...rest,
			"data-index": `item-${$.get(idx) ?? ""}`
		}));
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.animation(p, () => flip, null);
		$.append($$anchor, p);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
