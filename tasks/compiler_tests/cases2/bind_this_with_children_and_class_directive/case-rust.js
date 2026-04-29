import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span>x</span>`);
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	let dynamicEl;
	let counter = 0;
	var div = root();
	$.bind_this(div, ($$value) => dynamicEl = $$value, () => dynamicEl);
	$.set_class(div, 1, "", null, {}, { state: counter > 0 });
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if (counter) $$render(consequent);
		});
	}
	$.reset(div);
	$.append($$anchor, div);
}
