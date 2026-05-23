import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const first = $.mutable_source();
	const second = $.mutable_source();
	const total = $.mutable_source();
	const b = $.mutable_source();
	const d = $.mutable_source();
	const e = $.mutable_source();
	let source = $.prop($$props, "source", 8);
	$.legacy_pre_effect(() => ($.get(first), $.get(second), $.get(total), $.deep_read_state(source())), () => {
		(($$value) => {
			var $$array = $.to_array($$value.users, 2);
			$.set(first, $$array[0].name);
			$.set(second, $$array[1].name);
			$.set(total, $$value.total);
		})(source());
	});
	$.legacy_pre_effect(() => ($.get(b), $.get(d), $.get(e), $.deep_read_state(source())), () => {
		(($$value) => {
			var $$array_1 = $.to_array($$value.a, 2);
			var $$array_2 = $.to_array($$array_1[1].c, 2);
			$.set(b, $$array_1[0]);
			$.set(d, $$array_2[0]);
			$.set(e, $$array_2[1]);
		})(source());
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(first) ?? ""}-${$.get(second) ?? ""}-${$.get(total) ?? ""}-${$.get(b) ?? ""}-${$.get(d) ?? ""}-${$.get(e) ?? ""}`));
	$.append($$anchor, p);
	$.pop();
}
