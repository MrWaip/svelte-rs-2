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
	const c = $.mutable_source();
	const d = $.mutable_source();
	const s = writable({
		p: [1, 2],
		q: [3, 4]
	});
	$.legacy_pre_effect(() => ($.get(a), $.get(b), $.get(c), $.get(d), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value.p, 2);
			var $$array_1 = $.to_array($$value.q, 2);
			$.set(a, $$array[0]);
			$.set(b, $$array[1]);
			$.set(c, $$array_1[0]);
			$.set(d, $$array_1[1]);
		})($s());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
