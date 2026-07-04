import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Sticker from "./Sticker.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[9, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 24, () => []);
	const color = (x) => x.color ?? "red";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item) => {
		var fragment_1 = root();
		var node_1 = $.first_child(fragment_1);
		{
			let $0 = $.derived_safe_equal(() => ($.get(item), $.untrack(() => color($.get(item)))));
			$.css_props(node_1, () => ({ "--bg": $.get($0) }));
			Sticker(node_1.lastChild, $.spread_props(() => $.get(item)));
			$.reset(node_1);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 8, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
