import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { pick } from "./pick";
import { count } from "./count";
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let kind = $.prop($$props, "kind", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(pick), $.deep_read_state(kind()), $count(), $.untrack(() => pick(kind(), $count()))), $.index, ($$anchor, item) => {
		var div = root_1();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
