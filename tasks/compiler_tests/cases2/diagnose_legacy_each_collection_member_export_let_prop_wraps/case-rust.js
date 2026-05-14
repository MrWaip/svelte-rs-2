import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let item = $.prop($$props, "item", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(item()), $.untrack(() => item().list)), $.index, ($$anchor, entry) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $.get(entry)));
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
}
