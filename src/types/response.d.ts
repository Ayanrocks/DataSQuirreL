export interface IPCResponse<T> {
    status: number;
    error_code?: string;
    sys_err?: string;
    frontend_msg?: string;
    data?: T;
}

export interface StoredConnection {
    id: string,
    conn_name: string;
    host_name: string;
    database_name: string;
    database_type: string;
    port: number;
    user_name: string;
}

export interface TableSchema {
    table_catalog: string;
    table_name: string;
}

export interface TableData<T> {
    columns: string[];
    rows?: T[][];
    row_count?: string;
    table_name?: string;
    query_type: string;
}

export interface DashboardData {
    connection_data: Record<string, string>;
    dashboard_data: SchemaData[];
}