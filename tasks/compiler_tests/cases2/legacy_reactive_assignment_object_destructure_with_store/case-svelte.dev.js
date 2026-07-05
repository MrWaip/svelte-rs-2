import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $left = () => ($.validate_store($.get(left), "left"), $.store_get($.get(left), "$left", $$stores));
	const $right = () => ($.validate_store($.get(right), "right"), $.store_get($.get(right), "$right", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const left = $.mutable_source();
	const right = $.mutable_source();
	const renamed = $.mutable_source();
	const deep = $.mutable_source();
	let source = $.prop($$props, "source", 8);
	$.legacy_pre_effect(() => ($.get(left), $.get(right), $.get(renamed), $.get(deep), $.deep_read_state(source())), () => {
		(($$value) => {
			$.store_unsub($.set(left, $$value.left), "$left", $$stores);
			$.store_unsub($.set(right, $$value.right), "$right", $$stores);
			$.set(renamed, $$value.alias);
			$.set(deep, $$value.nested.deep);
		})(source());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$left() ?? ""}-${$right() ?? ""}-${$.get(renamed) ?? ""}-${$.get(deep) ?? ""}`));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
