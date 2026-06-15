import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const filtered = $.mutable_source();
	let items = [{ done: false }];
	$.legacy_pre_effect(() => {}, () => {
		$.set(filtered, items);
	});
	$.legacy_pre_effect_reset();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(filtered), $.index, ($$anchor, item) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, ($.get(item), $.untrack(() => $.get(item).done))));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	$.pop();
}
