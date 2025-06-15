<script lang="ts">
  import ExpandableArrowIcon from "../assets/icons/expandableArrow.svg?raw";
  import SchemaIcon from "../assets/icons/schema.svg?raw";
  import TableIcon from "../assets/icons/table.svg?raw";
  import PostgreSQLIcon from "../assets/icons/postgresql.svg?raw";

  let {
    entityName,
    isExpanded,
    entityType,
  }: { entityName: string; isExpanded: boolean; entityType: string } = $props();
</script>

<div class="sidebar-item">
  <div class="sidebar-item-content">
    <div class="expandable-icon scale-65">
      {@html isExpanded ? ExpandableArrowIcon : ExpandableArrowIcon}
    </div>
    <div class="entity-icon">
      {#if entityType === "Schema"}
        <span class="icon-container scale-85">
          {@html SchemaIcon}
        </span>
      {:else if entityType === "Table"}
        <span class="icon-container scale-85">
          {@html TableIcon}
        </span>
      {:else if entityType === "postgresql"}
        <span class="icon-container scale-85">
          {@html PostgreSQLIcon}
        </span>
        <!-- {:else if entityName === "View"}
        {@html ViewIcon}
      {:else if entityName === "Function"}
        {@html FunctionIcon} -->
      {/if}
    </div>
    <div class="entity-name">
      {entityName}
    </div>
  </div>
</div>

<style>
  .sidebar-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: start;
    width: 100%;
    height: 100%;
    color: #1d1b20;
  }

  .entity-name {
    font-size: 14px;
    font-weight: 500;
    color: #1d1b20;
    margin-left: 5px;
  }

  .sidebar-item-content {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: start;
    width: 100%;
    height: 100%;
  }

  .icon-container {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    color: #1d1b20;
    margin-left: 5px;
  }
</style>
