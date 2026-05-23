import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { LINKS } from "./links.js";
var root_1 = $.from_html(`<a> </a>`);
var root = $.from_html(`<h1> </h1> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const heading = $.mutable_source();
	let title = $.prop($$props, "title", 8, "");
	$.legacy_pre_effect(() => $.deep_read_state(title()), () => {
		$.set(heading, title().toUpperCase());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var fragment = root();
	var h1 = $.first_child(fragment);
	var text = $.child(h1, true);
	$.reset(h1);
	var node = $.sibling(h1, 2);
	$.each(node, 1, () => ($.deep_read_state(LINKS), $.untrack(() => LINKS.list)), $.index, ($$anchor, link) => {
		var a = root_1();
		var text_1 = $.child(a, true);
		$.reset(a);
		$.template_effect(() => {
			$.set_attribute(a, "href", ($.get(link), $.untrack(() => $.get(link).href)));
			$.set_text(text_1, ($.get(link), $.untrack(() => $.get(link).label)));
		});
		$.append($$anchor, a);
	});
	$.template_effect(() => $.set_text(text, $.get(heading)));
	$.append($$anchor, fragment);
	$.pop();
}
