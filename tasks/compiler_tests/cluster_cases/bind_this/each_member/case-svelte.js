import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let visible = $.prop($$props, "visible", 8, false);
	let items = $.prop($$props, "items", 24, () => [{
		value: "a",
		ref: null
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.each(node_1, 1, items, $.index, ($$anchor, item, $$index) => {
				var div = root_2();
				var text = $.child(div, true);
				$.reset(div);
				$.bind_this(div, ($$value, item) => (item.ref = $$value, $.invalidate_inner_signals(() => items())), (item) => item?.ref, () => [$.get(item)]);
				$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).value))));
				$.append($$anchor, div);
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (visible()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
