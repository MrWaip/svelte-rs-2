import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<span>a</span>`);
var root_1 = $.from_html(`<!> <p>tail</p>`, 1);
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	var node = $.child(div);
	Cmp(node, {
		children: ($$anchor, $$slotProps) => {
			var fragment = root_1();
			var node_1 = $.first_child(fragment);
			{
				var consequent = ($$anchor) => {
					var span = root_2();
					$.append($$anchor, span);
				};
				$.if(node_1, ($$render) => {
					if ($$props.x) $$render(consequent);
				});
			}
			$.next(2);
			$.append($$anchor, fragment);
		},
		$$slots: { default: true }
	});
	$.reset(div);
	$.append($$anchor, div);
}
