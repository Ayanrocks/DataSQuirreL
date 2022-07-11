<script>
  import { Grid } from 'ag-grid-community';
  import { onDestroy, onMount } from 'svelte';

  import 'ag-grid-community/dist/styles/ag-grid.css';
  import 'ag-grid-community/dist/styles/ag-theme-material.css';

  let domNode;
  let grid;

  var data = {
    columnDefs: [
      { field: 'athlete', width: 150 },
      { field: 'age', width: 90 },
      { field: 'country', width: 150 },
      { field: 'year', width: 90 },
      { field: 'date', width: 150 },
      { field: 'sport', width: 150 },
      { field: 'gold', width: 100 },
      { field: 'silver', width: 100 },
      { field: 'bronze', width: 100 },
      { field: 'total', width: 100 },
    ],
  };

  const gridOptions = {
    defaultColDef: {
      editable: true,
      enableRowGroup: true,
      enablePivot: true,
      enableValue: true,
      sortable: true,
      resizable: true,
      filter: true,
      flex: 1,
      minWidth: 100,
    },
    debounceVerticalScrollbar: true,
    autoSizePadding: 4,
    pagination: true,
    paginationPageSize: 3000,
    rowSelection: 'single',
    columnDefs: data.columnDefs,
    suppressColumnVirtualisation: true,
    suppressRowVirtualisation: true,
  };

  onMount(() => {
    grid = new Grid(domNode, gridOptions);

    fetch('https://www.ag-grid.com/example-assets/olympic-winners.json')
      .then((response) => response.json())
      .then((data) => gridOptions.api.setRowData(data));
  });

  onDestroy(() => {
    if (grid) {
      grid.destroy();
    }
  });
</script>

<div class="datatable-main-container">
  <div class="datagrid-container">
    <div id="datagrid" bind:this={domNode} class="ag-theme-material" />
  </div>
</div>

<style>
  .datatable-main-container {
    margin-top: 10px;
    width: 100%;
    height: 100%;
    padding: 10px;
  }

  .datagrid-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
  }

  #datagrid {
    height: 90%;
    width: 100%;
    --ag-header-foreground-color: blue;
  }
</style>
