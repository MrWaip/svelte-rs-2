App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p> <button>bump</button>`, 1), App[$.FILENAME], [[10, 0], [11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $held = () => ($.validate_store($.get(held), "held"), $.store_get($.get(held), "$held", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable({ bar: 0 });
	const held = $.tag($.derived(() => source), "held");
	function bump() {
		$.store_mutate($.get(held), $.untrack($held).bar = 6, $.untrack($held));
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $held().bar));
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
