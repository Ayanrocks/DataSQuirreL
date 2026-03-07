declare module "*.svg" {
  import type { Component } from "svelte";
  const component: Component<any>;
  export default component;
}

declare module "*.svg?raw" {
  const content: string;
  export default content;
}
