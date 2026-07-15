import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.each(node_1, 17, () => $$props.items, $.index, ($$anchor, item) => {
				var span = root();
				var text = $.child(span, true);
				$.reset(span);
				$.template_effect(() => $.set_text(text, $.get(item)));
				$.append($$anchor, span);
			});
			$.append($$anchor, fragment_1);
		};
		var alternate = ($$anchor) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.each(node_2, 17, () => $$props.items, $.index, ($$anchor, item) => {
				var div = root_1();
				var text_1 = $.child(div, true);
				$.reset(div);
				$.template_effect(() => $.set_text(text_1, $.get(item)));
				$.append($$anchor, div);
			});
			$.append($$anchor, fragment_2);
		};
		$.if(node, ($$render) => {
			if ($$props.loading) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
