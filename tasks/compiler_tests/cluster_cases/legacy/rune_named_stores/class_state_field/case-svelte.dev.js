import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	class Counter {
		value = $state()(0);
	}
	let c = $.tag($.mutable_source(new Counter()), "c");
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(c), $.untrack(() => $.get(c).value))));
	$.event("click", button, function click() {
		return $.mutate(c, $.get(c).value = $.get(c).value + 1);
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
