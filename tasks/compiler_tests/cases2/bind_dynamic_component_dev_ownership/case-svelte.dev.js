App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let value = $.prop($$props, "value", 15);
	let Comp = $.tag_proxy($.proxy(A), "Comp");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => Comp, ($$anchor, $$component) => {
		$$ownership_validator.binding("value", $$component, value);
		$$component($$anchor, {
			get value() {
				return value();
			},
			set value($$value) {
				value($$value);
			}
		});
	}), "component", App, 7, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
