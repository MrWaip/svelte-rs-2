import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy",
		"$$host"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, ["name"]);
	$.push($$props, false);
	let name = $.prop($$props, "name", 12);
	var $$exports = {
		get name() {
			return name();
		},
		set name($$value) {
			name($$value);
			$.flush();
		}
	};
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(($0) => {
		$.set_text(text, `name: ${name() ?? ""}`);
		$.set_text(text_1, $0);
	}, [() => ($.deep_read_state($$restProps), $.untrack(() => JSON.stringify($$restProps)))]);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
customElements.define("my-el", $.create_custom_element(App, { name: {} }, [], [], { mode: "open" }));
