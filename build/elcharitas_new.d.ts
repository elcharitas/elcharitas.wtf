/* tslint:disable */
/* eslint-disable */
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */

export type ReadableStreamType = "bytes";

export class ContainerStartupOptions {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    get enableInternet(): boolean | undefined;
    set enableInternet(value: boolean | null | undefined);
    entrypoint: string[];
    env: Map<any, any>;
}

export class IntoUnderlyingByteSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableByteStreamController): Promise<any>;
    start(controller: ReadableByteStreamController): void;
    readonly autoAllocateChunkSize: number;
    readonly type: ReadableStreamType;
}

export class IntoUnderlyingSink {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    abort(reason: any): Promise<any>;
    close(): Promise<any>;
    write(chunk: any): Promise<any>;
}

export class IntoUnderlyingSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableStreamDefaultController): Promise<any>;
}

/**
 * Configuration options for Cloudflare's minification features:
 * <https://www.cloudflare.com/website-optimization/>
 */
export class MinifyConfig {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    css: boolean;
    html: boolean;
    js: boolean;
}

export class R2Range {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    get length(): number | undefined;
    set length(value: number | null | undefined);
    get offset(): number | undefined;
    set offset(value: number | null | undefined);
    get suffix(): number | undefined;
    set suffix(value: number | null | undefined);
}

declare function fetch2(req: Request, env: any, ctx: any): Promise<Response>;
export { fetch2 as fetch }

export function scheduled(event: any, env: any, ctx: any): Promise<void>;

export function setPanicHook(callback: Function): void;
