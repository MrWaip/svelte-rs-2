import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<template shadowrootmode="closed"><p><slot></slot></p></template>`);
}
