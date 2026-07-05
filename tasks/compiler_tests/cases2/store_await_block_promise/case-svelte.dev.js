App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { promise } from "./stores";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 38]]);
var root_1 = $.add_locations($.from_html(`<p>p</p>`), App[$.FILENAME], [[5, 17]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $promise = () => ($.validate_store(promise, "promise"), $.store_get(promise, "$promise", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, $promise, ($$anchor) => {
		var p_1 = root_1();
		$.append($$anchor, p_1);
	}, ($$anchor, value) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, p);
	}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
