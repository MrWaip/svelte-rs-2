import * as $ from "svelte/internal/client";
import { isNotNil } from "es-toolkit";
import TransferFormLoading from "$apps/MFInternationalTransfersByPhone/views/TransferFormLoading.svelte";
import { contextifiedStore } from "../store/RootStore.svelte";
import { View } from "../store/views";
import TransferFormLoaded from "./TransferFormLoaded.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const rootStore = contextifiedStore.getStore();
	const { payoffLazy } = rootStore.navigation.useViewPayload(View.TransferForm);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			TransferFormLoaded($$anchor, {
				get payoffStore() {
					return payoffLazy.data;
				},
				set payoffStore($$value) {
					payoffLazy.data = $$value;
				}
			});
		};
		var d = $.derived(() => isNotNil(payoffLazy.data));
		var alternate = ($$anchor) => {
			TransferFormLoading($$anchor, {});
		};
		$.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
