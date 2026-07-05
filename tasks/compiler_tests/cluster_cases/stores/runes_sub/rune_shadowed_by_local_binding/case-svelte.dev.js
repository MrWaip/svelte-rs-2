App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = 5;
	let foo = $state()(0);
	let bar = $.tag($.derived(() => foo + 1), "bar");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${foo ?? ""} ${$.get(bar) ?? ""} 5`));
	$.delegated("click", button, function click() {
		return foo++;
	});
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
