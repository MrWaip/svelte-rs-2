import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let x = $.prop($$props, "x", 8);
	let y = $.prop($$props, "y", 12);
	let z = $.prop($$props, "z", 12);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => x() ? Foo : Bar, ($$anchor, $$component) => {
		$$ownership_validator.binding("y", $$component, y);
		$$ownership_validator.binding("z", $$component, z);
		$$component($$anchor, {
			get y() {
				return y();
			},
			set y($$value) {
				y($$value);
			},
			get z() {
				return z();
			},
			set z($$value) {
				z($$value);
			},
			$$legacy: true
		});
	}), "component", App, 8, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
