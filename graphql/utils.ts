import PageInfo from "./fragments/PageInfo.graphql";
import Post from "./fragments/Post.graphql";
import Publication from "./fragments/Publication.graphql";

type SingleEntryRecord<K extends string | number | symbol, V> = {
  [P in K]: Record<P, V> & Record<Exclude<K, P>, never>;
}[K];

type ApiResponse<T> = {
  data: T;
  errors?: Array<{
    message: string;
    locations: Array<{
      line: number;
      column: number;
    }>;
    path: Array<string | number>;
    extensions: {
      code: string;
    };
  }>;
};

export const buildQuery = (query: GraphQLQuery) => {
  const finalQuery = [query];
  if (query.includes("PageInfo")) {
    finalQuery.unshift(PageInfo);
  }
  if (query.includes("Post")) {
    finalQuery.unshift(Post);
  }
  if (query.includes("Publication")) {
    finalQuery.unshift(Publication);
  }
  return finalQuery.join("\n");
};

/**
 * Executes a list of similar GraphQL query against the given API endpoint.
 *
 * Sends a POST request with the query and variables to the provided base URL.
 * Parses the response as JSON and returns it.
 */
export const executeQuery = async <T>(
  /** The GraphQL query to execute */
  entry: SingleEntryRecord<string, GraphQLQuery>,

  /** Variables to pass to the query */
  variables?: Record<string, unknown>,

  /**
   * Options to configure the request.
   * @default { baseUrl: 'https://api.hashnode.com' }
   */
  options: {
    baseUrl?: string;
    headers?: Record<string, string>;
  } = {}
): Promise<ApiResponse<T>> => {
  const [[operationName, query]] = Object.entries(entry);

  const res = await fetch(options.baseUrl ?? "https://gql.hashnode.com", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
    body: JSON.stringify({
      query: buildQuery(query),
      variables,
      operationName,
    }),
  });

  return await res.json();
};
