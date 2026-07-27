import * as $ from "svelte/internal/client";
var root = $.from_html(`<template><!></template>`);
export default function App($$anchor, $$props) {
	var template = root();
	$.hydrate_template(template);
	var node = $.child(template.content);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($$props.cond) $$render(consequent);
		});
	}
	$.reset(template);
	$.append($$anchor, template);
}
