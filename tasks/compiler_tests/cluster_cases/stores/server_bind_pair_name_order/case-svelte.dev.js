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
			var bind_get = () => $$props.aGet();
			var bind_set = $$props.aSet;
			$.add_svelte_meta(() => Child($$anchor, {
				get value() {
					return bind_get();
				},
				set value($$value) {
					bind_set($$value);
				}
			}), "component", App, 7, 1, { componentTag: "Child" });
		};
		var alternate = ($$anchor) => {
			var bind_get_1 = () => $$props.bGet();
			var bind_set_1 = $$props.bSet;
			$.add_svelte_meta(() => Child($$anchor, {
				get value() {
					return bind_get_1();
				},
				set value($$value) {
					bind_set_1($$value);
				}
			}), "component", App, 9, 1, { componentTag: "Child" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.cond) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
