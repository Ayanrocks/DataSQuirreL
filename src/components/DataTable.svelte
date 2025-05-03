<script lang="ts">
  import { Grid } from 'ag-grid-community';
  import { onDestroy, onMount } from 'svelte';
  import DataTableToolBar from './DataTableToolBar.svelte';
  import { activeTable, notificationMsg } from '../stores';
  import type { ActiveTable } from '../stores'; // Import the ActiveTable type

  // import 'ag-grid-community/dist/styles/ag-grid.css';
  // import 'ag-grid-community/dist/styles/ag-theme-alpine.css';
  import { invoke } from '@tauri-apps/api/core';
  import { NOTIFICATION_TYPE_ERROR, PAGINATION_SIZE } from '../constants/constants';
  import type { GridOptions, GridApi, ColumnApi, ColDef } from 'ag-grid-community'; // Import necessary AG Grid types

  let domNode: HTMLElement;
  let grid: Grid;
  let gridApi: GridApi;
  let gridColumnApi: ColumnApi;
  let columnDefs: ColDef[] = [];
  let rowDefs: any[] = []; // Use 'any' for row data as the structure is dynamic
  let rowCount = 0;

  // props
  let tableData: ActiveTable = { // Initialize with default values matching the interface
    tableName: '',
    rows: [],
    columns: [],
    rowCount: 0,
    currentPage: 0,
    maxPage: 0,
  };

  activeTable.subscribe((val: ActiveTable) => { // Add type annotation for the subscribed value
    console.log('ActiveTable Subscribe', val);
    tableData = val;
  });

  onMount(() => {
    let gridOptions = {
      defaultColDef: {
        editable: true,
        sortable: true,
        resizable: true,
        filter: true,
        flex: 1,
        minWidth: 65,
      },
      debounceVerticalScrollbar: true,
      autoSizePadding: 2,
      // pagination: true, // Disable AG Grid's built-in pagination
      // paginationPageSize: PAGINATION_SIZE, // Disable AG Grid's built-in pagination
      columnDefs: columnDefs,
      rowData: rowDefs,
      suppressColumnVirtualisation: true,
      suppressRowVirtualisation: true,
      animateRows: false,
      infiniteInitialRowCount: 1,
      // suppressPaginationPanel: true, // Disable AG Grid's built-in pagination panel
      rowSelection: 'multiple',
      rowGroupPanelShow: 'always',
      pivotPanelShow: 'always',
      onGridReady: (params) => { // Move onGridReady inside gridOptions
        gridApi = params.api;
        gridColumnApi = params.columnApi;
      },
    } as GridOptions<any>; // Explicitly cast to GridOptions<any>

    grid = new Grid(domNode, gridOptions);
  });

  $: if (gridApi != null && tableData != null) {
    if (tableData.tableName !== '' && tableData.columns.length !== 0) {
      columnDefs = [];
      rowDefs = [];
      rowCount = tableData.rowCount;

      columnDefs.push({
        headerName: '#',
        valueGetter: 'node.id',
        pinned: 'left',
        lockPinned: true, // Change 'left' to true
        sortable: true,
        editable: false,
        cellEditorPopup: false,
        // comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB, // Remove incorrect comparator
      });

      // set the columns
      tableData.columns.forEach((elem) => {
        columnDefs.push({
          field: elem,
          headerName: elem,
          sortable: true,
          minWidth: 150,
          // comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB, // Remove incorrect comparator
        });
      });

      // set the rows
      tableData.rows.forEach((elem: any[]) => { // Add type annotation for elem
        let singleRowData: any = {}; // Explicitly type singleRowData as any
        elem.forEach((subElem: any, index: number) => { // Add type annotations for subElem and index
          singleRowData[tableData.columns[index] as string] = subElem; // Explicitly type index access
        });
        rowDefs.push(singleRowData);
      });

      gridApi.setColumnDefs(columnDefs);
      gridApi.setRowData(rowDefs);
      gridApi.sizeColumnsToFit();
    }
  }

  let gotoNext = () => {
    // calculate offset value
    let offsetVal = PAGINATION_SIZE * tableData.currentPage;
    fetchNextRecordBatch(tableData.tableName, offsetVal);
    gridApi.refreshClientSideRowModel('filter');
    // gridApi.paginationGoToNextPage(); // Remove AG Grid pagination call
  };

  let gotoPrev = () => {
    // gridApi.paginationGoToPreviousPage(); // Remove AG Grid pagination call
    // Need to implement logic to fetch previous batch if needed, or rely on already loaded data
    // For now, just remove the incorrect AG Grid call.
  };

  function fetchNextRecordBatch(tableName: string, offsetVal: number) { // Add type annotations
    console.log('TABLENAME: ', tableName);
    invoke<{ data: { rows: any[][], currentPage: number } }>('fetch_table_data_with_offset', { // Add type annotation for invoke result
      reqPayload: {
        table_name: tableName,
        offset: offsetVal,
      },
    })
      .then((res) => {
        console.log(res);
        let data = res.data;

        tableData.rows = [...data.rows];
        tableData.currentPage = data.currentPage; // Use data.currentPage from the response

        activeTable.set(tableData);
      })
      .catch((e: string) => { // Add type annotation for error
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: e,
        });
      });
  }

  onDestroy(() => {
    if (grid) {
      grid.destroy();
    }
  });
</script>

<div class="datatable-main-container">
  <div class="datagrid-container">
    <DataTableToolBar currentPage={tableData.currentPage} maxPage={tableData.maxPage} {gotoNext} {gotoPrev} />

    <div id="datagrid" bind:this={domNode} class="ag-theme-alpine"></div>
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
    height: 90%;
    position: relative;
    margin: 55px auto 10px auto;
  }

  #datagrid {
    margin-top: 5%;
    height: 90%;
    width: 100%;
    --ag-header-foreground-color: var(--accentColor);
  }
</style>
