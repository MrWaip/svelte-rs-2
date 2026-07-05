import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const handler_a = $.mutable_source();
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	$.legacy_pre_effect(() => {}, () => {
		$.set(handler_a, flag ? handler_1 : handler_2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, function(...$$args) {
		$.apply(() => $.get(handler_a), this, $$args, App, [8, 18]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
