import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="b svelte-13830z5"></div>`);
var root_1 = $.from_html(`<div class="c svelte-13830z5"></div>`);
var root_2 = $.from_html(`<div class="a svelte-13830z5"></div> <!> <!>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_2();
	var node = $.sibling($.first_child(fragment), 2);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if ($$props.x) $$render(consequent);
		});
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent_1 = ($$anchor) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		};
		$.if(node_1, ($$render) => {
			if ($$props.y) $$render(consequent_1);
		});
	}
	$.append($$anchor, fragment);
}
