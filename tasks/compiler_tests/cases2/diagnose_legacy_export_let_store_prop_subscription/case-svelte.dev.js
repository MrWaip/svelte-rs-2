import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <p> </p>`, 1), App[$.FILENAME], [[9, 0], [10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $challenge = () => ($.validate_store(challenge(), "challenge"), $.store_get(challenge(), "$challenge", $$stores));
	const $error = () => ($.validate_store(error(), "error"), $.store_get(error(), "$error", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const phone = $.mutable_source();
	let challenge = $.prop($$props, "challenge", 8);
	let error = $.prop($$props, "error", 8);
	$.legacy_pre_effect(() => $challenge(), () => {
		$.set(phone, $challenge()?.phone || "");
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var p = $.sibling(input, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(phone)));
	$.bind_value(input, function get() {
		return $error();
	}, function set($$value) {
		$.store_set(error(), $$value);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
