import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<style>span {
      color: green;
    }</style>`);
var root = $.from_html(`<div class="svelte-19xqvng"><style>.nested {
      color: red;
    }</style> <p class="nested">inside div</p></div> <!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	{
		var consequent = ($$anchor) => {
			var style = root_1();
			$.append($$anchor, style);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
