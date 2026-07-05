import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const store = $.tag($.mutable_source({ state: { show: true } }), "store");
	const close = () => {
		$.mutate(store, $.get(store).state.show = false);
	};
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(store), $.untrack(() => $.get(store).state.show))));
	$.delegated("click", button, close);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
