import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $challenge = () => $.store_get(challenge(), "$challenge", $$stores);
	const $error = () => $.store_get(error(), "$error", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const phone = $.mutable_source();
	let challenge = $.prop($$props, "challenge", 8);
	let error = $.prop($$props, "error", 8);
	$.legacy_pre_effect(() => $challenge(), () => {
		$.set(phone, $challenge()?.phone || "");
	});
	$.legacy_pre_effect_reset();
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var p = $.sibling(input, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(phone)));
	$.bind_value(input, $error, ($$value) => $.store_set(error(), $$value));
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
