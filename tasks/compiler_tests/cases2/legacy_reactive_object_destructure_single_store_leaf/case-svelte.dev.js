import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $value = () => ($.validate_store($.get(value), "value"), $.store_get($.get(value), "$value", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const value = $.mutable_source();
	let store = $.prop($$props, "store", 8);
	$.legacy_pre_effect(() => ($.get(value), $.deep_read_state(store())), () => {
		(($$value) => {
			$.store_unsub($.set(value, $$value.value), "$value", $$stores);
		})(store());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $value()));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
