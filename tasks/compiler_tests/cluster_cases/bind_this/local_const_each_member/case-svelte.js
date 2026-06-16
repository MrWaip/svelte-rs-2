import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	const items1 = $.mutable_source({});
	let data = [{
		id: 1,
		text: "a"
	}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => data, (item) => item.id, ($$anchor, item) => {
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, item) => $.mutate(items1, $.get(items1)[item.id] = $$value), (item) => $.get(items1)?.[item.id], () => [$.get(item)]);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).text))));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
