App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<template><!></template>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var template = root();
	$.hydrate_template(template);
	var node = $.child(template.content);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.cond) $$render(consequent);
		}), "if", App, 5, 10);
	}
	$.reset(template);
	$.append($$anchor, template);
	return $.pop($$exports);
}
