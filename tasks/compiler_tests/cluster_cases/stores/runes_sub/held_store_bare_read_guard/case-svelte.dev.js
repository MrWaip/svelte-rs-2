App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $held = () => ($.validate_store($.get(held), "held"), $.store_get($.get(held), "$held", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable(0);
	const held = $.tag($.derived(() => source), "held");
	function read() {
		const current = $held();
		return current;
	}
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => read()]);
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
