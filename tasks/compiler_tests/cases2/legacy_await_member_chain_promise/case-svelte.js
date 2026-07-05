import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let user = $.prop($$props, "user", 24, () => ({ fetch: () => Promise.resolve(null) }));
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => ($.deep_read_state(user()), $.untrack(() => user().fetch())), null, ($$anchor, v) => {
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
	$.pop();
}
