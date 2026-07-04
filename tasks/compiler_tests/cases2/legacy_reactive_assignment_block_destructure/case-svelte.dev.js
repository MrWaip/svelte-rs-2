import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[28, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const left = $.mutable_source();
	const right = $.mutable_source();
	let items = [{ value: 1 }, { value: 2 }];
	let source = {
		left: 3,
		right: 4
	};
	$.legacy_pre_effect(() => {}, () => {
		total = 0;
		for (const item of items) {
			total += item.value;
		}
	});
	$.legacy_pre_effect(() => ($.get(left), $.get(right)), () => {
		$.set(left, source.left), $.set(right, source.right);
	});
	$.legacy_pre_effect(() => {}, () => {
		if (items.length > 1) {
			conditional = total;
		} else {
			conditional = 0;
		}
	});
	$.legacy_pre_effect(() => ($.get(left), $.get(right)), () => {
		switch ($.get(left)) {
			case 3:
				switched = $.get(right);
				break;
			default: switched = 0;
		}
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${total ?? ""}-${$.get(left) ?? ""}-${$.get(right) ?? ""}-${conditional ?? ""}-${switched ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
