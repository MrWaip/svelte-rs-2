import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = 5;
	let foo = $.tag($.mutable_source($state()(0)), "foo");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""} 5`));
	$.delegated("click", button, function click() {
		return $.update(foo);
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
