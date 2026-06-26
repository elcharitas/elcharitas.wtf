/* tslint:disable */
/* eslint-disable */
export function scheduled(event: any, env: any, ctx: any): Promise<void>;
export function fetch(req: Request, env: any, ctx: any): Promise<Response>;
/**
 * Configuration options for Cloudflare's image optimization feature:
 * <https://blog.cloudflare.com/introducing-polish-automatic-image-optimizati/>
 */
export enum PolishConfig {
  Off = 0,
  Lossy = 1,
  Lossless = 2,
}
export enum RequestRedirect {
  Error = 0,
  Follow = 1,
  Manual = 2,
}
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */
type ReadableStreamType = "bytes";
export class IntoUnderlyingByteSource {
  private constructor();
  free(): void;
  pull(controller: ReadableByteStreamController): Promise<any>;
  start(controller: ReadableByteStreamController): void;
  cancel(): void;
  readonly autoAllocateChunkSize: number;
  readonly type: ReadableStreamType;
}
export class IntoUnderlyingSink {
  private constructor();
  free(): void;
  abort(reason: any): Promise<any>;
  close(): Promise<any>;
  write(chunk: any): Promise<any>;
}
export class IntoUnderlyingSource {
  private constructor();
  free(): void;
  pull(controller: ReadableStreamDefaultController): Promise<any>;
  cancel(): void;
}
/**
 * Configuration options for Cloudflare's minification features:
 * <https://www.cloudflare.com/website-optimization/>
 */
export class MinifyConfig {
  private constructor();
  free(): void;
  js: boolean;
  html: boolean;
  css: boolean;
}
export class R2Range {
  private constructor();
  free(): void;
  get offset(): number | undefined;
  set offset(value: number | null | undefined);
  get length(): number | undefined;
  set length(value: number | null | undefined);
  get suffix(): number | undefined;
  set suffix(value: number | null | undefined);
}
