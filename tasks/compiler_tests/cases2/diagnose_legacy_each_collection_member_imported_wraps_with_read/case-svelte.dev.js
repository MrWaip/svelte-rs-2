import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { LINKS } from "./links.js";
var root = $.add_locations($.from_html(`<a> </a>`), App[$.FILENAME], [[10, 4]]);
var root_1 = $.add_locations($.from_html(`<h1> </h1> <!>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const heading = $.mutable_source();
	let title = $.prop($$props, "title", 8, "");
	$.legacy_pre_effect(() => $.deep_read_state(title()), () => {
		$.set(heading, title().toUpperCase());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root_1();
	var h1 = $.first_child(fragment);
	var text = $.child(h1, true);
	$.reset(h1);
	var node = $.sibling(h1, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.deep_read_state(LINKS), $.untrack(() => LINKS.list)), $.index, ($$anchor, link) => {
		var a = root();
		var text_1 = $.child(a, true);
		$.reset(a);
		$.template_effect(() => {
			$.set_attribute(a, "href", ($.get(link), $.untrack(() => $.get(link).href)));
			$.set_text(text_1, ($.get(link), $.untrack(() => $.get(link).label)));
		});
		$.append($$anchor, a);
	}), "each", App, 9, 0);
	$.template_effect(() => $.set_text(text, $.get(heading)));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
