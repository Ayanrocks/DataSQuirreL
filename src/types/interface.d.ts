export interface SidebarItem {
  entityName: string;
  isExpanded: boolean;
  entityType: string;
  level: number;
  children: SidebarItem[];
}

export interface SchemaData {
  entityType: string;
  entityName: string;
  isExpanded: boolean;
  children?: SchemaData[];
}
