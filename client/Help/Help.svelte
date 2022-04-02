<script>

  // third-party components
  import LayoutGrid, { Cell } from '@smui/layout-grid';

  // local components
  import Menu from "../common/Menu.svelte"

  import Uptime from "./Uptime.svelte";
  import Journal from "./Journal.svelte";
  import Reboot from "./Reboot.svelte";
  import Reload from "./Reload.svelte";
  import Status from "./Status.svelte";
  import DiskSpace from "./DiskSpace.svelte";
  import Performance from "./Performance.svelte";
  import Launch from "./Launch.svelte";
  import Settings from "./Settings.svelte"

  let clicked = "";

  const menuClick = (e) => { 
    clicked = e.detail; 
    console.log("Help: menuClick invoked: ", clicked);
  }

</script>


<main class="main-content">
  {#if clicked == "Uptime"}
    <Uptime />
  {:else if clicked == "Journal"}
    <Journal />
  {:else if clicked == "Power Cycle"}
    <Reboot />
  {:else if clicked == "Reload"}
    <Reload />
  {:else if clicked == "Status"}
    <Status />
  {:else if clicked == "Disk Space"}
    <DiskSpace />    
  {:else if clicked == "Performance"}
    <Performance />
  {:else if clicked == "Launch Application"}
    <Launch />
  {:else if clicked == "Settings"}
    <Settings />
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
