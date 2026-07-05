import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const filtered = $.mutable_source();
	let items = [{ done: false }];
	$.legacy_pre_effect(() => {}, () => {
		$.set(filtered, items);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(filtered), $.index, ($$anchor, item) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).done))));
		$.append($$anchor, p);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
