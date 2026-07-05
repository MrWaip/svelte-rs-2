import * as $ from "svelte/internal/client";
var root = $.from_svg(`<g><path d="M1"></path></g>`);
var root_1 = $.from_svg(`<!><!>`, 1);
export default function App($$anchor) {
	let cond = true;
	let raw = "<circle r={5}/>";
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.html(node, () => raw, void 0, true);
	var node_1 = $.sibling(node);
	{
		var consequent = ($$anchor) => {
			var g = root();
			$.append($$anchor, g);
		};
		$.if(node_1, ($$render) => {
			if (cond) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
