App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p> <button>replace</button>`, 1), App[$.FILENAME], [[10, 0], [11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $held = () => ($.validate_store($.get(held), "held"), $.store_get($.get(held), "$held", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable(0);
	const held = $.tag($.derived(() => source), "held");
	function replace() {
		$.store_set($.get(held), writable(1));
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $held()));
	$.delegated("click", button, replace);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
