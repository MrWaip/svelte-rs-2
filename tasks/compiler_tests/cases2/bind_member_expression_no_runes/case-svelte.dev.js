App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { isNotNil } from "es-toolkit";
import TransferFormLoading from "$apps/MFInternationalTransfersByPhone/views/TransferFormLoading.svelte";
import { contextifiedStore } from "../store/RootStore.svelte";
import { View } from "../store/views";
import TransferFormLoaded from "./TransferFormLoaded.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const rootStore = contextifiedStore.getStore();
	const { payoffLazy } = rootStore.navigation.useViewPayload(View.TransferForm);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.validate_binding("bind:payoffStore={payoffLazy.data}", [], () => payoffLazy, () => "data", 15, 24);
			$.add_svelte_meta(() => TransferFormLoaded($$anchor, {
				get payoffStore() {
					return payoffLazy.data;
				},
				set payoffStore($$value) {
					payoffLazy.data = $$value;
				}
			}), "component", App, 15, 4, { componentTag: "TransferFormLoaded" });
		};
		var d = $.derived(() => isNotNil(payoffLazy.data));
		var alternate = ($$anchor) => {
			$.add_svelte_meta(() => TransferFormLoading($$anchor, {}), "component", App, 17, 4, { componentTag: "TransferFormLoading" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 14, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
