import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<p> </p> <p> </p>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = writable(0);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, $count());
		$.set_text(text_1, $state());
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
