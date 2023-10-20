interface GraphQLQuery extends String {
  // This is a strict typing of graphql file contents
  // Doing this ensures we do not confuse a graphql file with a string
}

declare module "*.graphql" {
  const value: GraphQLQuery;
  export default value;
}
