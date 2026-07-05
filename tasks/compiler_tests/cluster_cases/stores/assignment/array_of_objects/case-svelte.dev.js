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
	const b = $.mutable_source();
	const s = writable([{ a: 1 }, { b: 2 }]);
	$.legacy_pre_effect(() => ($.get(a), $.get(b), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(a, $$array[0].a);
			$.set(b, $$array[1].b);
		})($s());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
