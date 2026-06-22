import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<template id="t3">1<!>1</template>`);
export default function App($$anchor) {
	var template = root();
	$.hydrate_template(template);
	var node = $.sibling($.child(template.content));
	$.html(node, () => "<b>B</b>");
	$.next();
	$.reset(template);
	$.append($$anchor, template);
}
