import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $w = () => ($.validate_store(w, "w"), $.store_get(w, "$w", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const w = writable(0);
	let derived = $.tag($.mutable_source(0), "derived");
	$.legacy_pre_effect(() => {}, () => {
		(() => {
			$.store_set(w, 1);
		})();
	});
	$.legacy_pre_effect(() => $w(), () => {
		$.set(derived, $w() * 2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(derived)));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
