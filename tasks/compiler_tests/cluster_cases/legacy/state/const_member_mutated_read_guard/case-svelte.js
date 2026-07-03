import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const store = $.mutable_source({ state: { show: true } });
	const close = () => {
		$.mutate(store, $.get(store).state.show = false);
	};
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(store), $.untrack(() => $.get(store).state.show))));
	$.delegated("click", button, close);
	$.append($$anchor, button);
}
$.delegate(["click"]);
