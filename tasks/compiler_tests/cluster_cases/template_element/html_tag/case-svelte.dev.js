import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<template id="t3">1<!>1</template>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var template = root();
	$.hydrate_template(template);
	var node = $.sibling($.child(template.content));
	$.html(node, () => "<b>B</b>");
	$.next();
	$.reset(template);
	$.append($$anchor, template);
	return $.pop($$exports);
}
