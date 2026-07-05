import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const items1 = $.tag($.mutable_source({}), "items1");
	let data = [{
		id: 1,
		text: "a"
	}];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => data, (item) => item.id, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, item) => $.mutate(items1, $.get(items1)[item.id] = $$value), (item) => $.get(items1)?.[item.id], () => [$.get(item)]);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).text))));
		$.append($$anchor, div);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
