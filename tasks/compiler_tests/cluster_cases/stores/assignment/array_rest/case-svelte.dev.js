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
	const a = $.mutable_source();
	const rest = $.mutable_source();
	const s = writable([
		1,
		2,
		3
	]);
	$.legacy_pre_effect(() => ($.get(a), $.get(rest), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value);
			$.set(a, $$array[0]);
			$.set(rest, $$array.slice(1));
		})($s());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${($.get(rest), $.untrack(() => $.get(rest).length)) ?? ""}`));
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
