import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>x</div>`);
var select_content = $.from_html(`<!>`, 1);
var root_1 = $.from_html(`<select><!></select>`);
export default function App($$anchor, $$props) {
	var select = root_1();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var div = root();
				$.append($$anchor, div);
			};
			$.if(node, ($$render) => {
				if ($$props.show) $$render(consequent);
			});
		}
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
