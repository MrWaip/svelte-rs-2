App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const simpleReactive = $.tag($.derived(() => $$props.data.foo), "simpleReactive");
			$.get(simpleReactive);
			const computed_const = $.tag($.derived(() => {
				const { destr } = { destr: 1 };
				return { destr };
			}), "[@const]");
			$.get(computed_const);
			const simpleStatic = $.tag($.derived(() => 5), "simpleStatic");
			$.get(simpleStatic);
			$.add_svelte_meta(() => Child($$anchor, {
				get a() {
					return $.get(simpleReactive);
				},
				get b() {
					return $.get(computed_const).destr;
				},
				c: $.get(simpleStatic)
			}), "component", App, 9, 1, { componentTag: "Child" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.data) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
