declare module "*.svg" {
  import { SvelteComponentTyped } from "svelte";
  export default class extends SvelteComponentTyped<any, any, any> {}
}

declare module "*.svg?raw" {
  const content: string;
  export default content;
}
