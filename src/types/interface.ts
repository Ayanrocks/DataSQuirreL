export interface IPCResponse {
  status: number;
  error_code: string;
  sys_err: string;
  frontend_msg: string;
}

export interface FetchTableNamesResponse extends IPCResponse {
  data: {
    table_name: string[];
    columns: string[];
    row_count: string;
    rows: TableSchema[][];
    query_type: string;
  };
}

type TableSchema = {
  table_catalog: string;
  table_schema: string;
  table_name: string;
  table_type: string;
};

export interface IActiveTable {
  tableName: string;
  rows: never[][];
  columns: string[];
  rowCount: number;
  currentPage: number;
  maxPage: number;
}
