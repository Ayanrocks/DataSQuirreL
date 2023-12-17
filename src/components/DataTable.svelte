<script>
  import { Grid } from 'ag-grid-community';
  import AgGrid from '@budibase/svelte-ag-grid';
  import { onDestroy, onMount } from 'svelte';
  import DataTableToolBar from './DataTableToolBar.svelte';
  import { activeTable, notificationMsg } from '../stores';

  import { invoke } from '@tauri-apps/api/tauri';
  import { NOTIFICATION_TYPE_ERROR, PAGINATION_SIZE } from '../constants/constants';
  import DataTablePaginationBar from './DataTablePaginationBar.svelte';

  let domNode;
  let grid;
  let gridApi;
  let gridColumnApi;
  let columnDefs = [];
  let rowDefs = [];
  let rowCount = 0;

  // props
  let tableData = {};

  activeTable.subscribe((val) => {
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
      pagination: true,
      paginationPageSize: PAGINATION_SIZE,
      columnDefs: columnDefs,
      rowData: rowDefs,
      suppressColumnVirtualisation: true,
      suppressRowVirtualisation: true,
      animateRows: false,
      infiniteInitialRowCount: 1,
      suppressPaginationPanel: true,
      rowSelection: 'multiple',
      rowGroupPanelShow: 'always',
      pivotPanelShow: 'always',
    };

    grid = new Grid(domNode, gridOptions);
    gridOptions.onGridReady = (params) => {
      gridApi = params.api;
      gridColumnApi = params.columnApi;
    };
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
        lockPinned: 'left',
        sortable: true,
        editable: false,
        cellEditorPopup: false,
        comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB,
      });

      // set the columns
      tableData.columns.forEach((elem) => {
        columnDefs.push({
          field: elem,
          headerName: elem,
          sortable: true,
          minWidth: 150,
          comparator: (valueA, valueB, nodeA, nodeB, isDescending) => valueA - valueB,
        });
      });

      // set the rows
      tableData.rows.forEach((elem) => {
        let singleRowData = {};
        elem.forEach((subElem, index) => {
          singleRowData[tableData.columns[index]] = subElem;
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
    gridApi.paginationGoToNextPage();
  };

  let gotoPrev = () => {
    gridApi.paginationGoToPreviousPage();
  };

  function fetchNextRecordBatch(tableName, offsetVal) {
    console.debug('TABLENAME: ', tableName);
    invoke('fetch_table_data_with_offset', {
      reqPayload: {
        table_name: tableName,
        offset: offsetVal,
      },
    })
      .then((res) => {
        console.log(res);
        let data = res.data;

        tableData.rows = [...data.rows];
        tableData.currentPage = tableData.currentPage + 1;

        activeTable.set(tableData);
      })
      .catch((e) => {
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: e,
        });
      });
  }

  onDestroy(() => {});
</script>

<div class="datatable-main-container">
  <div class="datagrid-container">
    <DataTableToolBar currentPage={tableData.currentPage} maxPage={tableData.maxPage} {gotoNext} {gotoPrev} />

    <div id="datagrid" bind:this={domNode} class="ag-theme-alpine"></div>
    <DataTablePaginationBar currentPage={tableData.currentPage} maxPage={tableData.maxPage} {gotoNext} {gotoPrev} />
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
    flex-direction: row;
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
