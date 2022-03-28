<script>
  import LayoutGrid, { Cell } from '@smui/layout-grid';

  // Components
  import Menu from "../common/Menu.svelte"
  import DiskSpace from "./DiskSpace.svelte";
  import Journal from "./Journal.svelte";
  import Status from "./Status.svelte";
  import Uptime from "./Uptime.svelte";
  import Reboot from "./Reboot.svelte";
  import Launch from "./Launch.svelte";
  import Performance from "./Performance.svelte";

  let clicked = "";

  const menuClick = (e) => { 
    clicked = e.detail; 
    console.log("Help: menuClick invoked: ", clicked);
  }

</script>

<main class="main-content">
    {#if clicked == "Disk Space"}
      <DiskSpace />
    {:else if clicked == "Uptime"}
      <Uptime />
    {:else if clicked == "Status"}
      <Status />
    {:else if clicked == "Journal"}
      <Journal />
    {:else if clicked == "Performance"}
      <Performance />
    {:else if clicked == "Power Cycle"}
      <Reboot />    
    {:else if clicked == "Launch Application"}
      <Launch />
    {:else if clicked == ""}
    <LayoutGrid>
      <Menu on:menuClick={menuClick} />
    </LayoutGrid>
    {:else}
      <p>you clicked {clicked}</p>
    {/if}
</main>


<style>

  .main-content {
    /*float: right;*/
    overflow: auto;
    padding: 16px;
    height: 100%;
    box-sizing: border-box;
  }
  
</style>
