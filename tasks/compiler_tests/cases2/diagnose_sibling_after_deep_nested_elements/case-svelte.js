import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div><div><div><div><span>a</span></div></div></div> <div><!></div></div>`);
export default function App($$anchor) {
	const inner = ($$anchor, mf = $.noop) => {
		var div = root_1();
		var div_1 = $.sibling($.child(div), 2);
		var node = $.child(div_1);
		$.key(node, () => x, ($$anchor) => {
			var fragment = $.comment();
			var node_1 = $.first_child(fragment);
			$.snippet(node_1, mf);
			$.append($$anchor, fragment);
		});
		$.reset(div_1);
		$.reset(div);
		$.append($$anchor, div);
	};
	let x = 0;
}
