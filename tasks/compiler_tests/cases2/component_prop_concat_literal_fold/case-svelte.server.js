import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
export default function App($$renderer) {
	Button($$renderer, { label: "value: 1231" });
}
