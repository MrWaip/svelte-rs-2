import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
export default function App($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Button($$renderer, $.spread_props([props]));
}
