import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let items = $.prop($$props, "items", 25, () => [1, 2]);
	let user = $.prop($$props, "user", 25, () => ({ name: "a" }));
	$.init(true);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, ($.deep_read_state(items()), $.untrack(() => items().length)));
		$.set_text(text_1, ($.deep_read_state(user()), $.untrack(() => user().name)));
	});
	$.append($$anchor, fragment);
	$.pop();
}
