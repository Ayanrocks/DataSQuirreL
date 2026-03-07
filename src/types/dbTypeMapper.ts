export interface IDbTypeMapper {
  sqlToJsType(sqlType: string): string;
}
