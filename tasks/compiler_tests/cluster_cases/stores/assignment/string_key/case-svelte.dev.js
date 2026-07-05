import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $s = () => ($.validate_store(s, "s"), $.store_get(s, "$s", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const ab = $.mutable_source();
	const cd = $.mutable_source();
	const s = writable({
		"a-b": 1,
		"c d": 2
	});
	$.legacy_pre_effect(() => ($.get(ab), $.get(cd), $s()), () => {
		(($$value) => {
			$.set(ab, $$value["a-b"]);
			$.set(cd, $$value["c d"]);
		})($s());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
