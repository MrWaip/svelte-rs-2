import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	class Counter {
		value = $state()(0);
	}
	let c = $.mutable_source(new Counter());
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(c), $.untrack(() => $.get(c).value))));
	$.event("click", button, () => $.mutate(c, $.get(c).value = $.get(c).value + 1));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
