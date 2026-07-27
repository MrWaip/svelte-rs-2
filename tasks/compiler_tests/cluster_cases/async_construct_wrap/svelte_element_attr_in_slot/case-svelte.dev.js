import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function f() {
		return 1;
	}
	async function g() {
		return 2;
	}
	var $$exports = { ...$.legacy_api() };
	{
		$.async($$anchor, void 0, [async () => (await $.track_reactivity_loss(f()))()], ($$anchor, $0) => {
			$.add_svelte_meta(() => Child($$anchor, {
				get a() {
					return $.get($0);
				},
				children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
					var fragment_1 = $.comment();
					var node = $.first_child(fragment_1);
					{
						$.validate_dynamic_element_tag(() => $$props.tag);
						$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
							$.attribute_effect($$element, ($0) => ({ title: $0 }), void 0, [async () => (await $.track_reactivity_loss(g()))()]);
						}, void 0, [8, 1]);
					}
					$.append($$anchor, fragment_1);
				}),
				$$slots: { default: true }
			}), "component", App, 7, 0, { componentTag: "Child" });
		});
		$.next();
	}
	return $.pop($$exports);
}
