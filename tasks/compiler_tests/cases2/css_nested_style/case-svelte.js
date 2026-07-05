import * as $ from "svelte/internal/client";
var root = $.from_html(`<style>span {
      color: green;
    }</style>`);
var root_1 = $.from_html(`<div class="svelte-19xqvng"><style>.nested {
      color: red;
    }</style> <p class="nested">inside div</p></div> <!>`, 1);
export default function App($$anchor) {
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	{
		var consequent = ($$anchor) => {
			var style = root();
			$.append($$anchor, style);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
