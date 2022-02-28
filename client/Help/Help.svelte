<script>

  // third-party components
  import LayoutGrid, { Cell } from '@smui/layout-grid';

  // local components
  import Menu from "../common/Menu.svelte"
  import DiskSpace from "./DiskSpace.svelte";
  import Status from "./Status.svelte";

  let clicked = "";

  const menuClick = (e) => { 
    clicked = e.detail; 
    console.log("Help: menuClick invoked: ", clicked);
  }

</script>


<main class="main-content">
  {#if clicked == "Disk Space"}
    <DiskSpace />
  {:else if clicked == "Status"}
    <Status />
  {:else if clicked == ""}
  <LayoutGrid>
    <Menu on:menuClick={menuClick} />
  </LayoutGrid>
  {:else}
    <p>you clicked {clicked}</p>
  {/if}
</main>


<style>

  * :global(.app-content) {
    flex: auto;
    overflow: auto;
    position: relative;
    flex-grow: 1;
  }

  .main-content {
    overflow: auto;
    padding: 16px;
    height: 100%;
    box-sizing: border-box;
  }

</style>
