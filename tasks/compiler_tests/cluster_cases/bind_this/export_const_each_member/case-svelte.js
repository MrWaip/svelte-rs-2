import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const items1 = $.mutable_source({});
	let data = $.prop($$props, "data", 24, () => [{
		id: 1,
		text: "b"
	}]);
	var $$exports = {
		get items1() {
			return $.get(items1);
		},
		set items1($$value) {
			$.set(items1, $.proxy($$value));
		}
	};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, data, (item) => item.id, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, item) => $.mutate(items1, $.get(items1)[item.id] = $$value), (item) => $.get(items1)?.[item.id], () => [$.get(item)]);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).text))));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.bind_prop($$props, "items1", $.get(items1));
	return $.pop($$exports);
}
