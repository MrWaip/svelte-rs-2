App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $value = () => ($.validate_store(value, "value"), $.store_get(value, "$value", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const value = writable({
		a: 1,
		b: 2
	});
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.tag($.derived(() => {
				const { a, b } = $value();
				return {
					a,
					b
				};
			}), "[@const]");
			$.get(computed_const);
			$.add_svelte_meta(() => Comp($$anchor, {
				get x() {
					return $.get(computed_const).a;
				},
				get b() {
					return $.get(computed_const).b;
				}
			}), "component", App, 9, 1, { componentTag: "Comp" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($value()) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
