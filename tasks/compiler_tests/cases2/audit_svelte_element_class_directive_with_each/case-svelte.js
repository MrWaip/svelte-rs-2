import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "div";
	let items = $.proxy([{
		id: 1,
		active: false
	}, {
		id: 2,
		active: true
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, (item) => item.id, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.element(node_1, () => tag, false, ($$element, $$anchor) => {
			let classes;
			$.template_effect(() => classes = $.set_class($$element, 0, "", null, classes, { active: $.get(item).active }));
			var text = $.text();
			$.template_effect(() => $.set_text(text, $.get(item).id));
			$.append($$anchor, text);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
